use std::path::PathBuf;

use crate::traits::FromReader;

/// An adapter for automatically loading the contents of a file path
#[derive(Debug, Clone)]
pub struct PathTo<T> {
    /// The path given as an argument by the user
    pub path: PathBuf,

    /// The data extracted from the file at the path
    pub data: T,
}

impl<T: FromReader> std::str::FromStr for PathTo<T> {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        let file = std::fs::File::open(&path)?;
        let mut reader = std::io::BufReader::new(file);
        let data = T::from_reader(&mut reader)?;
        let item = PathTo { path, data };
        Ok(item)
    }
}
