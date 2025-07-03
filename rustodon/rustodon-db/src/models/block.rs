//! Block model for Rustodon
//!
//! This module defines the Block model and its database operations.
//! It handles user blocking relationships.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use crate::error::DbError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info, trace};

/// Block model representing a user blocking another user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: i64,
    pub blocker_id: i64,
    pub blocked_id: i64,
    pub created_at: Option<NaiveDateTime>,
}

impl Block {
    /// Get all blocks
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        trace!("Fetching all blocks");
        let blocks = sqlx::query_as!(
            Block,
            "SELECT id, blocker_id, blocked_id, created_at FROM blocks ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} blocks", blocks.len());
        Ok(blocks)
    }

    /// Get blocks by account (where account is the blocker)
    pub async fn get_by_account(pool: &PgPool, account_id: i64) -> Result<Vec<Self>, DbError> {
        trace!("Fetching blocks for account: {}", account_id);
        let blocks = sqlx::query_as!(
            Block,
            "SELECT id, blocker_id, blocked_id, created_at FROM blocks WHERE blocker_id = $1 ORDER BY created_at DESC",
            account_id
        )
        .fetch_all(pool)
        .await?;

        info!(
            "Fetched {} blocks for account: {}",
            blocks.len(),
            account_id
        );
        Ok(blocks)
    }

    /// Create a new block
    pub async fn create(pool: &PgPool, blocker_id: i64, blocked_id: i64) -> Result<Self, DbError> {
        trace!("Creating block: {} blocks {}", blocker_id, blocked_id);
        let block = sqlx::query_as!(
            Block,
            "INSERT INTO blocks (blocker_id, blocked_id) VALUES ($1, $2) RETURNING id, blocker_id, blocked_id, created_at",
            blocker_id,
            blocked_id
        )
        .fetch_one(pool)
        .await?;

        info!("Created block: {} blocks {}", blocker_id, blocked_id);
        Ok(block)
    }

    /// Remove a block
    pub async fn remove(pool: &PgPool, blocker_id: i64, blocked_id: i64) -> Result<bool, DbError> {
        trace!("Removing block: {} unblocks {}", blocker_id, blocked_id);
        let result = sqlx::query!(
            "DELETE FROM blocks WHERE blocker_id = $1 AND blocked_id = $2",
            blocker_id,
            blocked_id
        )
        .execute(pool)
        .await?;

        let removed = result.rows_affected() > 0;
        if removed {
            info!("Removed block: {} unblocks {}", blocker_id, blocked_id);
        } else {
            debug!(
                "Block not found for removal: {} -> {}",
                blocker_id, blocked_id
            );
        }
        Ok(removed)
    }

    /// Check if a block exists
    pub async fn exists(pool: &PgPool, blocker_id: i64, blocked_id: i64) -> Result<bool, DbError> {
        trace!("Checking if block exists: {} -> {}", blocker_id, blocked_id);
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM blocks WHERE blocker_id = $1 AND blocked_id = $2",
            blocker_id,
            blocked_id
        )
        .fetch_one(pool)
        .await?;

        let exists = count.count.unwrap_or(0) > 0;
        debug!(
            "Block exists: {} -> {} = {}",
            blocker_id, blocked_id, exists
        );
        Ok(exists)
    }

    /// Delete a block
    pub async fn delete(pool: &PgPool, account_id: i64, target_id: i64) -> Result<bool, DbError> {
        use tracing::{error, info};
        info!("Deleting block: {} blocks {}", account_id, target_id);
        let result = sqlx::query!(
            "DELETE FROM blocks WHERE blocker_id = $1 AND blocked_id = $2",
            account_id,
            target_id
        )
        .execute(pool)
        .await;
        match result {
            Ok(res) => Ok(res.rows_affected() > 0),
            Err(e) => {
                error!("Error deleting block: {}", e);
                Err(DbError::from(e))
            }
        }
    }
}
