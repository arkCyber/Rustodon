//! Mute model for Rustodon
//!
//! This module defines the Mute model and its database operations.
//! It handles user muting relationships.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use crate::error::DbError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info, trace};

/// Mute model representing a user muting another user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mute {
    pub id: i64,
    pub muter_id: i64,
    pub muted_id: i64,
    pub hide_notifications: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
}

impl Mute {
    /// Get all mutes
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        trace!("Fetching all mutes");
        let mutes = sqlx::query_as!(
            Mute,
            "SELECT id, muter_id, muted_id, hide_notifications, created_at FROM mutes ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} mutes", mutes.len());
        Ok(mutes)
    }

    /// Get mutes by account (where account is the muter)
    pub async fn get_by_account(pool: &PgPool, account_id: i64) -> Result<Vec<Self>, DbError> {
        trace!("Fetching mutes for account: {}", account_id);
        let mutes = sqlx::query_as!(
            Mute,
            "SELECT id, muter_id, muted_id, hide_notifications, created_at FROM mutes WHERE muter_id = $1 ORDER BY created_at DESC",
            account_id
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} mutes for account: {}", mutes.len(), account_id);
        Ok(mutes)
    }

    /// Create a new mute
    pub async fn create(
        pool: &PgPool,
        muter_id: i64,
        muted_id: i64,
        hide_notifications: Option<bool>,
    ) -> Result<Self, DbError> {
        trace!("Creating mute: {} mutes {}", muter_id, muted_id);
        let mute = sqlx::query_as!(
            Mute,
            "INSERT INTO mutes (muter_id, muted_id, hide_notifications) VALUES ($1, $2, $3) RETURNING id, muter_id, muted_id, hide_notifications, created_at",
            muter_id,
            muted_id,
            hide_notifications
        )
        .fetch_one(pool)
        .await?;

        info!("Created mute: {} mutes {}", muter_id, muted_id);
        Ok(mute)
    }

    /// Remove a mute
    pub async fn remove(pool: &PgPool, muter_id: i64, muted_id: i64) -> Result<bool, DbError> {
        trace!("Removing mute: {} unmutes {}", muter_id, muted_id);
        let result = sqlx::query!(
            "DELETE FROM mutes WHERE muter_id = $1 AND muted_id = $2",
            muter_id,
            muted_id
        )
        .execute(pool)
        .await?;

        let removed = result.rows_affected() > 0;
        if removed {
            info!("Removed mute: {} unmutes {}", muter_id, muted_id);
        } else {
            debug!("Mute not found for removal: {} -> {}", muter_id, muted_id);
        }
        Ok(removed)
    }

    /// Check if a mute exists
    pub async fn exists(pool: &PgPool, muter_id: i64, muted_id: i64) -> Result<bool, DbError> {
        trace!("Checking if mute exists: {} -> {}", muter_id, muted_id);
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM mutes WHERE muter_id = $1 AND muted_id = $2",
            muter_id,
            muted_id
        )
        .fetch_one(pool)
        .await?;

        let exists = count.count.unwrap_or(0) > 0;
        debug!("Mute exists: {} -> {} = {}", muter_id, muted_id, exists);
        Ok(exists)
    }

    /// Delete a mute
    pub async fn delete(pool: &PgPool, account_id: i64, target_id: i64) -> Result<bool, DbError> {
        use tracing::{error, info};
        info!("Deleting mute: {} mutes {}", account_id, target_id);
        let result = sqlx::query!(
            "DELETE FROM mutes WHERE muter_id = $1 AND muted_id = $2",
            account_id,
            target_id
        )
        .execute(pool)
        .await;
        match result {
            Ok(res) => Ok(res.rows_affected() > 0),
            Err(e) => {
                error!("Error deleting mute: {}", e);
                Err(DbError::from(e))
            }
        }
    }
}
