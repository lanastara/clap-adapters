//! Adapter types for declaratively loading configurations

#![warn(missing_docs)]

/// Adapters for reading file contents from CLI paths
pub mod fs;

/// Adapters for reading http resources from CLI urls
pub mod http;

/// Adapters for parsing JSON documents
pub mod json;

/// Adapters for auto-reloading file contents on change
pub mod reload;

/// Adapters for parsing TOML documents
pub mod toml;

/// Traits for glueing adapters together
pub mod traits;

/// Adapters for parsing YAML documents
pub mod yaml;

/// Convenience import for clap adapter building blocks
pub mod prelude {
    pub use crate::fs::*;
    pub use crate::http::*;
    pub use crate::json::*;
    pub use crate::reload::*;
    pub use crate::toml::*;
    pub use crate::traits::*;
    pub use crate::yaml::*;
}
