use serde::de::DeserializeOwned;

use crate::traits::FromReader;

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
