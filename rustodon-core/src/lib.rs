//!
//! Rustodon Core Module
//!
//! This crate provides core types, traits, and utilities shared across the Rustodon server backend.
//! It defines global error types, common data structures, and foundational traits for other modules.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_core::RustodonVersion;
//! let v = RustodonVersion::current();
//! println!("Rustodon version: {}", v);
//! ```
//!
//! # Dependencies
//!
//! - `tracing`: Structured logging
//! - `serde`: Serialization
//! - `thiserror`: Error handling
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::debug;

// Module declarations
pub mod error;

// Re-export error types
pub use error::{ContextualError, ErrorContext, ErrorResponse, RustodonError, RustodonResult};

/// Global error type for Rustodon core operations
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Rustodon version information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RustodonVersion {
    /// Version string (e.g., "0.1.0")
    pub version: String,
}

impl RustodonVersion {
    /// Returns the current version of Rustodon
    ///
    /// # Examples
    ///
    /// ```
    /// use rustodon_core::RustodonVersion;
    /// let v = RustodonVersion::current();
    /// assert_eq!(v.version, "0.1.0");
    /// ```
    pub fn current() -> Self {
        debug!("Fetching Rustodon version");
        Self {
            version: "0.1.0".to_string(),
        }
    }
}

impl std::fmt::Display for RustodonVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_version() {
        let v = RustodonVersion::current();
        assert_eq!(v.version, "0.1.0");
    }
}
