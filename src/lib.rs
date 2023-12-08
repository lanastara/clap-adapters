//! Adapter types for declaratively loading configurations

#![warn(missing_docs)]

use std::path::PathBuf;

use reqwest::{header::USER_AGENT, Url};
use serde::de::DeserializeOwned;

mod reload;
pub use reload::Reloading;

/// Any type that can construct itself from a buffered reader
pub trait FromReader: Sized {
    /// The kind of error that may occur during construction
    type Error: std::error::Error + Send + Sync + 'static;

    /// How the type constructs itself from a buffered reader
    fn from_reader(reader: &mut impl std::io::BufRead) -> Result<Self, Self::Error>;
}

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

/// An adapter for automatically loading contents from an Http Get endpoint
#[derive(Debug, Clone)]
pub struct HttpGet<T> {
    /// The URL of the GET request to make
    pub url: Url,

    /// The data received from the request
    pub data: T,
}

impl<T: FromReader> std::str::FromStr for HttpGet<T> {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = s.parse::<Url>()?;
        // let res = reqwest::blocking::get(url.clone())?;
        let res = reqwest::blocking::Client::new()
            .get(url.clone())
            .header(USER_AGENT, env!("CARGO_CRATE_NAME"))
            .send()?;
        let text = res.text()?;
        let mut reader = std::io::Cursor::new(text);
        let data = T::from_reader(&mut reader)?;
        let item = HttpGet { url, data };
        Ok(item)
    }
}

impl FromReader for Vec<u8> {
    type Error = std::io::Error;
    fn from_reader(reader: &mut impl std::io::BufRead) -> Result<Self, Self::Error> {
        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

impl FromReader for String {
    type Error = std::io::Error;
    fn from_reader(reader: &mut impl std::io::BufRead) -> Result<Self, Self::Error> {
        let mut string = String::new();
        reader.read_to_string(&mut string)?;
        Ok(string)
    }
}

/// An adapter for deserializing a Json document from a buffered reader
#[derive(Debug, Clone)]
pub struct JsonOf<T>(pub T);
impl<T: DeserializeOwned> FromReader for JsonOf<T> {
    type Error = serde_json::Error;
    fn from_reader(reader: &mut impl std::io::BufRead) -> Result<Self, Self::Error> {
        let json = serde_json::from_reader::<_, T>(reader)?;
        Ok(JsonOf(json))
    }
}

/// An adapter for deserializing a Yaml document from a buffered reader
#[derive(Debug, Clone)]
pub struct YamlOf<T>(pub T);
impl<T: DeserializeOwned> FromReader for YamlOf<T> {
    type Error = serde_yaml::Error;
    fn from_reader(reader: &mut impl std::io::BufRead) -> Result<Self, Self::Error> {
        let yaml = serde_yaml::from_reader(reader)?;
        Ok(YamlOf(yaml))
    }
}

/// An adapter for deserializing a Toml document from a buffered reader
#[derive(Debug, Clone)]
pub struct TomlOf<T>(pub T);
impl<T: DeserializeOwned> FromReader for TomlOf<T> {
    type Error = std::io::Error;
    fn from_reader(reader: &mut impl std::io::BufRead) -> Result<Self, Self::Error> {
        let string = String::from_reader(reader)?;
        let toml = toml::from_str(&string)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
        Ok(TomlOf(toml))
    }
}
