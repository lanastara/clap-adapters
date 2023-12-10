use std::sync::Arc;

use notify::{FsEventWatcher, RecursiveMode};
use tokio::sync::watch;
use tokio_stream::Stream;

use crate::fs::PathTo;
use crate::traits::FromReader;

/// An adapter for reloading a filesystem document when it's changed
#[derive(Clone)]
#[must_use = "Dropping the `Reloading` will cancel the file watch"]
pub struct Reloading<T> {
    reload_rx: watch::Receiver<T>,
    _watcher: Arc<FsEventWatcher>,
}

impl<T: Clone> Reloading<T> {
    /// Get the current value of the inner document
    pub fn get(&self) -> T {
        self.reload_rx.borrow().clone()
    }

    /// Get a receiver channel that yields updated documents after filesystem changes
    pub fn receiver(&self) -> watch::Receiver<T> {
        self.reload_rx.clone()
    }
}

impl<T: Clone + Send + Sync + 'static> Reloading<T> {
    /// Get a stream of document changes
    pub fn stream(&self) -> impl Stream<Item = T> {
        tokio_stream::wrappers::WatchStream::new(self.reload_rx.clone())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Reloading<PathTo<T>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Reloading")
            .field(&*self.reload_rx.borrow())
            .finish()
    }
}

impl<T: FromReader + Clone + Send + Sync + 'static> std::str::FromStr for Reloading<PathTo<T>> {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use notify::Watcher;
        let path_to = PathTo::<T>::from_str(s)?;
        let (reload_tx, reload_rx) = watch::channel(path_to.clone());

        let path = path_to.path.clone();
        let mut watcher = notify::recommended_watcher(move |result| {
            if let Err(error) = result {
                tracing::warn!(
                    error = format!("{error:#}"),
                    "Notify triggered with error, skipping"
                );
            }

            // Attempt to re-open file and read it into our typed format
            let data_result = (|| -> anyhow::Result<T> {
                let file = std::fs::File::open(&path)?;
                let mut reader = std::io::BufReader::new(file);
                let data = T::from_reader(&mut reader)?;
                Ok(data)
            })();

            let data = match data_result {
                Ok(data) => data,
                Err(error) => {
                    tracing::error!(
                        error = format!("{error:#}"),
                        path = %path.display(),
                        "Failed to hotreload after notify",
                    );
                    return;
                }
            };

            let updated_path_to = PathTo {
                path: path.clone(),
                data,
            };

            reload_tx.send_replace(updated_path_to);
        })?;
        watcher.watch(&path_to.path, RecursiveMode::NonRecursive)?;

        let item = Self {
            reload_rx,
            _watcher: Arc::new(watcher),
        };
        Ok(item)
    }
}
