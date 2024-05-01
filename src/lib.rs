//! Adapter types for declaratively loading configurations
//!
//! Did you know that any type that implements [`FromStr`] can
//! be used in a [`clap`] derive struct? That means that any
//! logic you can fit into a `fn(&str) -> Result<T, Error>` can
//! be run at parsing-time. This can be expecially useful for
//! declaratively selecting config files or doing other cool
//! stuff. Check this out:
//!
//! ```
//! # fn main() -> anyhow::Result<()> {
//! use clap::Parser;
//! use clap_adapters::prelude::*;
//!
//! #[derive(Debug, Parser)]
//! struct Cli {
//!     /// Path to a config file of arbitrary Json
//!     #[clap(long)]
//!     config: PathTo<JsonOf<serde_json::Value>>,
//! }
//!
//! // Create a config file in a temporary directory
//! let config_dir = tempfile::tempdir()?;
//! let config_path = config_dir.path().join("config.json");
//! let config_path_string = config_path.display().to_string();
//!
//! // Write a test config of {"hello":"world"} to the config file
//! let config = serde_json::json!({"hello": "world"});
//! let config_string = serde_json::to_string(&config)?;
//! std::fs::write(&config_path, &config_string)?;
//!
//! // Parse our CLI, passing our config file path to --config
//! let cli = Cli::parse_from(["app", "--config", &config_path_string]);
//! let data = cli.config.data();
//!
//! // We should expect the value we get to match what we wrote to the config
//! assert_eq!(data, &serde_json::json!({"hello":"world"}));
//! # Ok(())
//! # }
//! ```
//!
//! [`FromStr`]: std::str::FromStr

#![warn(missing_docs)]

/// Adapters for reading file contents from CLI paths
mod fs;

/// Adapters for parsing JSON documents
mod json;

/// Adapter for reloading file contents periodically
#[cfg(any(doc, feature = "periodic"))]
mod periodic;

#[cfg(any(doc, feature = "reloading"))]
mod reloading;

/// Adapters for parsing TOML documents
mod toml;

/// Traits for glueing adapters together
pub mod traits;

/// Adapters for parsing YAML documents
mod yaml;

pub use {fs::PathTo, json::JsonOf, toml::TomlOf, yaml::YamlOf};

#[cfg(any(doc, feature = "periodic"))]
pub use periodic::Periodic;

#[cfg(any(doc, feature = "reloading"))]
pub use reloading::Reloading;

/// Convenience import for clap adapter building blocks
pub mod prelude {
    pub use crate::fs::*;
    pub use crate::json::*;
    #[cfg(any(doc, feature = "periodic"))]
    pub use crate::periodic::*;
    #[cfg(any(doc, feature = "reloading"))]
    pub use crate::reloading::*;
    pub use crate::toml::*;
    pub use crate::traits::*;
    pub use crate::yaml::*;
}
