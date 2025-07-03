//! Error types for IP blocking functionality
//!
//! This module defines error types used throughout the IP blocking system.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use cidr::errors::NetworkParseError;
use sqlx::Error as SqlxError;
use thiserror::Error;

/// Error type for IP blocking operations
#[derive(Error, Debug)]
pub enum IpBlockError {
    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    /// Invalid IP address format
    #[error("Invalid IP address format: {0}")]
    InvalidIpAddress(String),

    /// Invalid CIDR range format
    #[error("Invalid CIDR range format: {0}")]
    InvalidCidrRange(String),

    /// IP block not found
    #[error("IP block not found: {0}")]
    NotFound(String),

    /// IP block already exists
    #[error("IP block already exists: {0}")]
    AlreadyExists(String),

    /// Invalid severity level
    #[error("Invalid severity level: {0}")]
    InvalidSeverity(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<std::net::AddrParseError> for IpBlockError {
    fn from(err: std::net::AddrParseError) -> Self {
        IpBlockError::InvalidIpAddress(err.to_string())
    }
}

impl From<NetworkParseError> for IpBlockError {
    fn from(err: NetworkParseError) -> Self {
        IpBlockError::InvalidCidrRange(err.to_string())
    }
}
