use serde::de::DeserializeOwned;

use crate::prelude::FromReader;

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
