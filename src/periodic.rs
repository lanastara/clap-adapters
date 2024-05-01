//! Provides the [`Periodic`] adapter for loading files at a regular interval

use std::time::Duration;

use tokio::sync::watch;
use tokio_stream::Stream;

use crate::{prelude::FromReader, PathTo};

/// Given a [`Path`] from the user, provides a utility that reloads the file
/// at the path at a fixed interval
///
/// - Use [`Periodic::get`] to get the file contents at a given moment
/// - Use [`Periodic::receiver`] to get a tokio [`watch::Receiver`]
///
/// # Example
///
/// ```no_run
/// use clap::Parser;
/// use clap_adapters::prelude::*;
///
/// #[derive(Debug, Parser)]
/// struct Cli {
///     /// Path to a Json config to be reloaded every 24 hours
///     #[clap(long)]
///     daily_config: Periodic<PathTo<JsonOf<serde_json::Value>>, Hours<24>>,
///
///     /// Path to a Json config to be reloaded every minute
///     #[clap(long)]
///     minutely_config: Periodic<PathTo<YamlOf<serde_json::Value>>>, // Minutes<1> is the default period
///
///     /// Path to a Json config to be reloaded every second
///     #[clap(long)]
///     secondly_config: Periodic<PathTo<TomlOf<serde_json::Value>>, Seconds<1>>,
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let cli = Cli::parse_from([
///         "app",
///         "--daily_config=./daily_config.json",
///         "--minutely-config=./minutely_config.yaml",
///         "--secondly-config=./secondly_config.toml",
///     ]);
///    
///     let current_config = cli.daily_config.get();
///     let current_config = cli.minutely_config.get();
///     let current_config = cli.secondly_config.get();
///    
///     let daily_config_rx = cli.daily_config.receiver();
///     let minutely_config_rx = cli.minutely_config.receiver();
///     let secondly_config_rx = cli.secondly_config.receiver();
/// }
/// ```
///
/// > *Note*: [`Periodic`] requires a tokio runtime to be active before calling
/// > any of the `clap::Parser` functions
///
/// [`Path`]: std::path::Path
/// [`watch::Receiver`]: tokio::sync::watch
#[derive(Clone)]
#[must_use = "Dropping the `Periodic` will cancel the file watch"]
pub struct Periodic<T, P: Time = Minutes<1>> {
    reload_rx: watch::Receiver<T>,
    period: std::marker::PhantomData<P>,
}

impl<T: Clone, P: Time> Periodic<T, P> {
    /// Get the current value of the inner document
    pub fn get(&self) -> T {
        self.reload_rx.borrow().clone()
    }

    /// Get a receiver channel that yields updated documents after filesystem changes
    pub fn receiver(&self) -> watch::Receiver<T> {
        self.reload_rx.clone()
    }
}

impl<T, P: Time> Periodic<T, P>
where
    T: Clone + Send + Sync + 'static,
{
    /// Get a stream of document changes
    pub fn stream(&self) -> impl Stream<Item = T> {
        tokio_stream::wrappers::WatchStream::new(self.reload_rx.clone())
    }
}

impl<T, P: Time> std::fmt::Debug for Periodic<PathTo<T>, P>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Periodic")
            .field(&*self.reload_rx.borrow())
            .finish()
    }
}

impl<T, P: Time> std::str::FromStr for Periodic<PathTo<T>, P>
where
    T: FromReader + Clone + Send + Sync + 'static,
{
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path_to = PathTo::<T>::from_str(s)?;
        let (reload_tx, reload_rx) = watch::channel(path_to.clone());

        tokio::spawn(async move {
            let path = path_to.path;
            let reload_tx = reload_tx;

            loop {
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
                            "Failed to reload after time period",
                        );
                        continue;
                    }
                };

                let updated_path_to = PathTo {
                    path: path.clone(),
                    data,
                };

                reload_tx.send_replace(updated_path_to);
                tokio::time::sleep(P::PERIOD).await;
            }
        });

        let item = Self {
            reload_rx,
            period: std::marker::PhantomData,
        };
        Ok(item)
    }
}

/// Trait for type markers to const-evaluate to a Duration
pub trait Time {
    /// The duration between periodic file reloads
    const PERIOD: Duration;
}

/// Reload the file every `N` seconds
#[derive(Debug, Clone, Copy)]
pub enum Seconds<const N: u64> {}
impl<const N: u64> Time for Seconds<N> {
    const PERIOD: Duration = Duration::from_secs(N);
}

/// Reload the file every `N` minutes
#[derive(Debug, Clone, Copy)]
pub enum Minutes<const N: u64> {}
impl<const N: u64> Time for Minutes<N> {
    const PERIOD: Duration = Duration::from_secs(60 * N);
}

/// Reload the file every `N` hours
#[derive(Debug, Clone, Copy)]
pub enum Hours<const N: u64> {}
impl<const N: u64> Time for Hours<N> {
    const PERIOD: Duration = Duration::from_secs(60 * 60 * N);
}
