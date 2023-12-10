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

impl<T> crate::fs::PathTo<JsonOf<T>> {
    /// Returns reference to the inner JSON datatype
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> anyhow::Result<()> {
    /// use clap::Parser;
    /// use clap_adapters::prelude::*;
    ///
    /// #[derive(Debug, Parser)]
    /// struct Cli {
    ///     #[clap(long)]
    ///     config: PathTo<JsonOf<serde_json::Value>>,
    /// }
    ///
    /// // Create a config file in a temporary directory
    /// let config_dir = tempfile::tempdir()?;
    /// let config_path = config_dir.path().join("config.json");
    /// let config_path_string = config_path.display().to_string();
    ///
    /// // Write a test config of {"hello":"world"} to the config file
    /// let config = serde_json::json!({"hello": "world"});
    /// let config_string = serde_json::to_string(&config)?;
    /// std::fs::write(&config_path, &config_string)?;
    ///
    /// // Parse our CLI, passing our config file path to --config
    /// let cli = Cli::parse_from(["app", "--config", &config_path_string]);
    /// let json = serde_json::to_string(cli.config.json())?;
    ///
    /// // We should expect the value we get to match what we wrote to the config
    /// assert_eq!(&json, r#"{"hello":"world"}"#);
    /// # Ok(())
    /// # }
    /// ```
    pub fn json(&self) -> &T {
        &self.data.0
    }
}
