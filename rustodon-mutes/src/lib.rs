//! Mutes module for Rustodon
//!
//! This module provides mute (user block) functionality for the Rustodon server.
//! It handles creating, managing, and querying mutes between users with proper database operations and validation.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_mutes::Mute;
//!
//! let mute = Mute::create(&pool, muter_id, muted_id, Some(true)).await?;
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

/// Custom error type for mutes module
#[derive(Error, Debug)]
pub enum MutesError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Mute not found")]
    MuteNotFound,
    #[error("User not found: {0}")]
    UserNotFound(i64),
    #[error("Already muted")]
    AlreadyMuted,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Mute data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mute {
    /// Unique identifier for the mute
    pub id: i64,
    /// ID of the muter (the user who mutes)
    pub muter_id: i64,
    /// ID of the muted (the user who is muted)
    pub muted_id: i64,
    /// Whether to hide notifications from muted user
    pub hide_notifications: bool,
    /// When the mute was created
    pub created_at: DateTime<Utc>,
}

impl Mute {
    /// Creates a new mute
    pub async fn create(
        pool: &PgPool,
        muter_id: i64,
        muted_id: i64,
        hide_notifications: Option<bool>,
    ) -> Result<Self, MutesError> {
        trace!("Creating mute: {} -> {}", muter_id, muted_id);
        if muter_id == muted_id {
            return Err(MutesError::Validation("Cannot mute yourself".to_string()));
        }
        // Check if already muted
        let exists = sqlx::query_scalar!(
            r#"SELECT 1 FROM mutes WHERE muter_id = $1 AND muted_id = $2"#,
            muter_id,
            muted_id
        )
        .fetch_optional(pool)
        .await?;
        if exists.is_some() {
            return Err(MutesError::AlreadyMuted);
        }
        // Insert mute
        let row = sqlx::query!(
            r#"INSERT INTO mutes (muter_id, muted_id, hide_notifications)
            VALUES ($1, $2, $3)
            RETURNING id, muter_id, muted_id, hide_notifications, created_at"#,
            muter_id,
            muted_id,
            hide_notifications.unwrap_or(false)
        )
        .fetch_one(pool)
        .await?;
        info!("Mute created: {} -> {}", muter_id, muted_id);
        Ok(Mute {
            id: row.id,
            muter_id: row.muter_id,
            muted_id: row.muted_id,
            hide_notifications: row.hide_notifications,
            created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
        })
    }
    /// Deletes a mute
    pub async fn delete(pool: &PgPool, muter_id: i64, muted_id: i64) -> Result<(), MutesError> {
        trace!("Deleting mute: {} -> {}", muter_id, muted_id);
        let result = sqlx::query!(
            r#"DELETE FROM mutes WHERE muter_id = $1 AND muted_id = $2"#,
            muter_id,
            muted_id
        )
        .execute(pool)
        .await?;
        if result.rows_affected() == 0 {
            return Err(MutesError::MuteNotFound);
        }
        info!("Mute deleted: {} -> {}", muter_id, muted_id);
        Ok(())
    }
    /// Checks if a mute exists
    pub async fn exists(pool: &PgPool, muter_id: i64, muted_id: i64) -> Result<bool, MutesError> {
        trace!("Checking mute exists: {} -> {}", muter_id, muted_id);
        let exists = sqlx::query_scalar!(
            r#"SELECT 1 FROM mutes WHERE muter_id = $1 AND muted_id = $2"#,
            muter_id,
            muted_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(exists.is_some())
    }
    /// Gets all mutes for an account
    pub async fn get_by_account(pool: &PgPool, muter_id: i64) -> Result<Vec<Self>, MutesError> {
        trace!("Getting mutes for account: {}", muter_id);
        let rows = sqlx::query!(
            r#"SELECT id, muter_id, muted_id, hide_notifications, created_at
            FROM mutes WHERE muter_id = $1 ORDER BY created_at DESC"#,
            muter_id
        )
        .fetch_all(pool)
        .await?;
        let mutes = rows
            .into_iter()
            .map(|row| Mute {
                id: row.id,
                muter_id: row.muter_id,
                muted_id: row.muted_id,
                hide_notifications: row.hide_notifications,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
            })
            .collect();
        Ok(mutes)
    }
    /// Gets all mutes
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, MutesError> {
        trace!("Getting all mutes");
        let rows = sqlx::query!(
            r#"SELECT id, muter_id, muted_id, hide_notifications, created_at
            FROM mutes ORDER BY created_at DESC"#
        )
        .fetch_all(pool)
        .await?;
        let mutes = rows
            .into_iter()
            .map(|row| Mute {
                id: row.id,
                muter_id: row.muter_id,
                muted_id: row.muted_id,
                hide_notifications: row.hide_notifications,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
            })
            .collect();
        Ok(mutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mute_struct() {
        let mute = Mute {
            id: 1,
            muter_id: 1,
            muted_id: 2,
            hide_notifications: true,
            created_at: Utc::now(),
        };
        assert_eq!(mute.muter_id, 1);
        assert_eq!(mute.muted_id, 2);
        assert!(mute.hide_notifications);
    }
}
