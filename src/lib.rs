//! Adapter types for declaratively loading configurations

#![warn(missing_docs)]

mod fs;
mod http;
mod json;
mod reload;
mod toml;
mod traits;
mod yaml;

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
