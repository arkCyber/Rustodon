//! Bookmarks module for Rustodon
//!
//! This module provides bookmark functionality for the Rustodon server.
//! It handles creating, managing, and querying bookmarks for statuses with proper database operations and validation.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_bookmarks::Bookmark;
//!
//! let bookmark = Bookmark::create(&pool, account_id, status_id).await?;
//! ```
//!
//! # Dependencies
//!
//! - `sqlx`: Database queries
//! - `serde`: Serialization
//! - `chrono`: DateTime handling
//! - `thiserror`: Error handling
//! - `tracing`: Logging
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;
use tracing::{error, info, trace};

/// Custom error type for bookmarks module
#[derive(Error, Debug)]
pub enum BookmarksError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Bookmark not found")]
    BookmarkNotFound,
    #[error("Already bookmarked")]
    AlreadyBookmarked,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Bookmark data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    /// Unique identifier for the bookmark
    pub id: i64,
    /// ID of the account that bookmarked
    pub account_id: i64,
    /// ID of the status that was bookmarked
    pub status_id: i64,
    /// When the bookmark was created
    pub created_at: DateTime<Utc>,
}

impl Bookmark {
    /// Creates a new bookmark
    pub async fn create(
        pool: &PgPool,
        account_id: i64,
        status_id: i64,
    ) -> Result<Self, BookmarksError> {
        trace!("Creating bookmark: {} -> {}", account_id, status_id);
        // Check if already bookmarked
        let exists = sqlx::query_scalar!(
            r#"SELECT 1 FROM bookmarks WHERE account_id = $1 AND status_id = $2"#,
            account_id,
            status_id
        )
        .fetch_optional(pool)
        .await?;
        if exists.is_some() {
            return Err(BookmarksError::AlreadyBookmarked);
        }
        // Insert bookmark
        let row = sqlx::query!(
            r#"INSERT INTO bookmarks (account_id, status_id)
            VALUES ($1, $2)
            RETURNING id, account_id, status_id, created_at"#,
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?;
        info!("Bookmark created: {} -> {}", account_id, status_id);
        Ok(Bookmark {
            id: row.id,
            account_id: row.account_id,
            status_id: row.status_id,
            created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
        })
    }
    /// Deletes a bookmark
    pub async fn delete(
        pool: &PgPool,
        account_id: i64,
        status_id: i64,
    ) -> Result<(), BookmarksError> {
        trace!("Deleting bookmark: {} -> {}", account_id, status_id);
        let result = sqlx::query!(
            r#"DELETE FROM bookmarks WHERE account_id = $1 AND status_id = $2"#,
            account_id,
            status_id
        )
        .execute(pool)
        .await?;
        if result.rows_affected() == 0 {
            return Err(BookmarksError::BookmarkNotFound);
        }
        info!("Bookmark deleted: {} -> {}", account_id, status_id);
        Ok(())
    }
    /// Checks if a bookmark exists
    pub async fn exists(
        pool: &PgPool,
        account_id: i64,
        status_id: i64,
    ) -> Result<bool, BookmarksError> {
        trace!("Checking bookmark exists: {} -> {}", account_id, status_id);
        let exists = sqlx::query_scalar!(
            r#"SELECT 1 FROM bookmarks WHERE account_id = $1 AND status_id = $2"#,
            account_id,
            status_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(exists.is_some())
    }
    /// Gets all bookmarks for an account
    pub async fn get_by_account(
        pool: &PgPool,
        account_id: i64,
    ) -> Result<Vec<Self>, BookmarksError> {
        trace!("Getting bookmarks for account: {}", account_id);
        let rows = sqlx::query!(
            r#"SELECT id, account_id, status_id, created_at
            FROM bookmarks WHERE account_id = $1 ORDER BY created_at DESC"#,
            account_id
        )
        .fetch_all(pool)
        .await?;
        let bookmarks = rows
            .into_iter()
            .map(|row| Bookmark {
                id: row.id,
                account_id: row.account_id,
                status_id: row.status_id,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
            })
            .collect();
        Ok(bookmarks)
    }
    /// Gets all bookmarks
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, BookmarksError> {
        trace!("Getting all bookmarks");
        let rows = sqlx::query!(
            r#"SELECT id, account_id, status_id, created_at
            FROM bookmarks ORDER BY created_at DESC"#
        )
        .fetch_all(pool)
        .await?;
        let bookmarks = rows
            .into_iter()
            .map(|row| Bookmark {
                id: row.id,
                account_id: row.account_id,
                status_id: row.status_id,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
            })
            .collect();
        Ok(bookmarks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bookmark_struct() {
        let bookmark = Bookmark {
            id: 1,
            account_id: 1,
            status_id: 2,
            created_at: Utc::now(),
        };
        assert_eq!(bookmark.account_id, 1);
        assert_eq!(bookmark.status_id, 2);
    }
}
