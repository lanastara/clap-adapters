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

impl<T> crate::fs::PathTo<YamlOf<T>> {
    /// Returns reference to the inner YAML datatype
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
    ///     config: PathTo<YamlOf<serde_json::Value>>,
    /// }
    ///
    /// // Create a config file in a temporary directory
    /// let config_dir = tempfile::tempdir()?;
    /// let config_path = config_dir.path().join("config.json");
    /// let config_path_string = config_path.display().to_string();
    ///
    /// // Write a test config to the config file
    /// let config_string = r#"hello: "world""#;
    /// std::fs::write(&config_path, &config_string)?;
    ///
    /// // Parse our CLI, passing our config file path to --config
    /// let cli = Cli::parse_from(["app", "--config", &config_path_string]);
    /// let yaml = serde_yaml::to_string(cli.config.yaml())?;
    ///
    /// // We should expect the value we get to match what we wrote to the config
    /// assert_eq!(&yaml, "hello: world\n");
    /// # Ok(())
    /// # }
    /// ```
    pub fn yaml(&self) -> &T {
        &self.data.0
    }
}
