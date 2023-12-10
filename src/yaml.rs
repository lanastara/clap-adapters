use serde::de::DeserializeOwned;

use crate::prelude::FromReader;

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
