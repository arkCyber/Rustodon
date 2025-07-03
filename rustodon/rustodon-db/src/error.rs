//! Database error types for Rustodon
//!
//! This module defines custom error types for database operations.
//! It provides meaningful error messages and proper error handling.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use thiserror::Error;

/// Custom error type for database operations
#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),
    #[error("Internal error: {0}")]
    Internal(String),
}
