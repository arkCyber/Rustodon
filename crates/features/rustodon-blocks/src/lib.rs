//! Blocks module for Rustodon
//!
//! This module provides functionality for managing user blocks in the Rustodon server.
//! It handles creating, removing, and querying blocks between users.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_blocks::Block;
//!
//! let block = Block::create(&pool, blocker_id, blocked_id).await?;
//! ```
//!
//! # Dependencies
//!
//! - `rustodon_core`: Core types and traits
//! - `rustodon_db`: Database operations
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;
use tracing::{debug, error, info, trace};

/// Custom error type for blocks module
#[derive(Error, Debug)]
pub enum BlocksError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Block not found")]
    BlockNotFound,
    #[error("User not found: {0}")]
    UserNotFound(i64),
    #[error("Cannot block yourself")]
    CannotBlockSelf,
    #[error("Already blocked")]
    AlreadyBlocked,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Block data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Unique identifier for the block
    pub id: i64,
    /// ID of the user doing the blocking
    pub blocker_id: i64,
    /// ID of the user being blocked
    pub blocked_id: i64,
    /// When the block was created
    pub created_at: DateTime<Utc>,
}

impl Block {
    /// Creates a new block
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `blocker_id` - ID of the user doing the blocking
    /// * `blocked_id` - ID of the user being blocked
    ///
    /// # Returns
    ///
    /// Result containing the created block or an error
    pub async fn create(
        pool: &PgPool,
        blocker_id: i64,
        blocked_id: i64,
    ) -> Result<Self, BlocksError> {
        trace!("Creating block from {} to {}", blocker_id, blocked_id);

        // Check if trying to block yourself
        if blocker_id == blocked_id {
            return Err(BlocksError::CannotBlockSelf);
        }

        // Check if both users exist
        let blocker_exists = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM users
            WHERE id = $1
            "#,
            blocker_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if blocker_exists == Some(0) {
            return Err(BlocksError::UserNotFound(blocker_id));
        }

        let blocked_exists = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM users
            WHERE id = $1
            "#,
            blocked_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if blocked_exists == Some(0) {
            return Err(BlocksError::UserNotFound(blocked_id));
        }

        // Check if already blocked
        let existing_block = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM blocks
            WHERE blocker_id = $1 AND blocked_id = $2
            "#,
            blocker_id,
            blocked_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if existing_block > Some(0) {
            return Err(BlocksError::AlreadyBlocked);
        }

        // Insert block
        let block_row = sqlx::query!(
            r#"
            INSERT INTO blocks (blocker_id, blocked_id)
            VALUES ($1, $2)
            RETURNING id, blocker_id, blocked_id, created_at
            "#,
            blocker_id,
            blocked_id
        )
        .fetch_one(pool)
        .await?;

        let block = Block {
            id: block_row.id,
            blocker_id: block_row.blocker_id,
            blocked_id: block_row.blocked_id,
            created_at: DateTime::from_naive_utc_and_offset(
                block_row.created_at.expect("created_at should not be null"),
                Utc,
            ),
        };

        info!(
            "Created block with ID: {} from {} to {}",
            block.id, blocker_id, blocked_id
        );
        Ok(block)
    }

    /// Removes a block
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `blocker_id` - ID of the user who did the blocking
    /// * `blocked_id` - ID of the user who was blocked
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn remove(
        pool: &PgPool,
        blocker_id: i64,
        blocked_id: i64,
    ) -> Result<(), BlocksError> {
        trace!("Removing block from {} to {}", blocker_id, blocked_id);

        let result = sqlx::query!(
            r#"
            DELETE FROM blocks
            WHERE blocker_id = $1 AND blocked_id = $2
            "#,
            blocker_id,
            blocked_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(BlocksError::BlockNotFound);
        }

        info!("Removed block from {} to {}", blocker_id, blocked_id);
        Ok(())
    }

    /// Checks if a block exists
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `blocker_id` - ID of the user who did the blocking
    /// * `blocked_id` - ID of the user who was blocked
    ///
    /// # Returns
    ///
    /// Result containing true if blocked, false otherwise
    pub async fn exists(
        pool: &PgPool,
        blocker_id: i64,
        blocked_id: i64,
    ) -> Result<bool, BlocksError> {
        trace!(
            "Checking if block exists from {} to {}",
            blocker_id,
            blocked_id
        );

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM blocks
            WHERE blocker_id = $1 AND blocked_id = $2
            "#,
            blocker_id,
            blocked_id
        )
        .fetch_one(pool)
        .await?
        .count;

        Ok(count > Some(0))
    }

    /// Gets all blocks for a user
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `blocker_id` - ID of the user who did the blocking
    /// * `limit` - Maximum number of blocks to return
    /// * `since_id` - Return blocks after this ID
    /// * `max_id` - Return blocks before this ID
    ///
    /// # Returns
    ///
    /// Result containing the list of blocks or an error
    pub async fn get_by_blocker(
        pool: &PgPool,
        blocker_id: i64,
        limit: Option<i64>,
        since_id: Option<i64>,
        max_id: Option<i64>,
    ) -> Result<Vec<Self>, BlocksError> {
        trace!(
            "Getting blocks for blocker {} with limit {:?}",
            blocker_id,
            limit
        );

        let limit = limit.unwrap_or(20).min(40);

        let block_rows = if let Some(since_id) = since_id {
            sqlx::query_as!(
                BlockRow,
                r#"
                SELECT id, blocker_id, blocked_id, created_at
                FROM blocks
                WHERE blocker_id = $1 AND id > $2
                ORDER BY id ASC
                LIMIT $3
                "#,
                blocker_id,
                since_id,
                limit
            )
            .fetch_all(pool)
            .await?
        } else if let Some(max_id) = max_id {
            sqlx::query_as!(
                BlockRow,
                r#"
                SELECT id, blocker_id, blocked_id, created_at
                FROM blocks
                WHERE blocker_id = $1 AND id < $2
                ORDER BY id DESC
                LIMIT $3
                "#,
                blocker_id,
                max_id,
                limit
            )
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as!(
                BlockRow,
                r#"
                SELECT id, blocker_id, blocked_id, created_at
                FROM blocks
                WHERE blocker_id = $1
                ORDER BY id DESC
                LIMIT $2
                "#,
                blocker_id,
                limit
            )
            .fetch_all(pool)
            .await?
        };

        let blocks: Vec<Block> = block_rows
            .into_iter()
            .map(|row| Block {
                id: row.id,
                blocker_id: row.blocker_id,
                blocked_id: row.blocked_id,
                created_at: DateTime::from_naive_utc_and_offset(
                    row.created_at.expect("created_at should not be null"),
                    Utc,
                ),
            })
            .collect();

        debug!(
            "Retrieved {} blocks for blocker {}",
            blocks.len(),
            blocker_id
        );
        Ok(blocks)
    }

    /// Gets all users who blocked a specific user
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `blocked_id` - ID of the user who was blocked
    /// * `limit` - Maximum number of blocks to return
    /// * `since_id` - Return blocks after this ID
    /// * `max_id` - Return blocks before this ID
    ///
    /// # Returns
    ///
    /// Result containing the list of blocks or an error
    pub async fn get_by_blocked(
        pool: &PgPool,
        blocked_id: i64,
        limit: Option<i64>,
        since_id: Option<i64>,
        max_id: Option<i64>,
    ) -> Result<Vec<Self>, BlocksError> {
        trace!(
            "Getting blocks for blocked user {} with limit {:?}",
            blocked_id,
            limit
        );

        let limit = limit.unwrap_or(20).min(40);

        let block_rows = if let Some(since_id) = since_id {
            sqlx::query_as!(
                BlockRow,
                r#"
                SELECT id, blocker_id, blocked_id, created_at
                FROM blocks
                WHERE blocked_id = $1 AND id > $2
                ORDER BY id ASC
                LIMIT $3
                "#,
                blocked_id,
                since_id,
                limit
            )
            .fetch_all(pool)
            .await?
        } else if let Some(max_id) = max_id {
            sqlx::query_as!(
                BlockRow,
                r#"
                SELECT id, blocker_id, blocked_id, created_at
                FROM blocks
                WHERE blocked_id = $1 AND id < $2
                ORDER BY id DESC
                LIMIT $3
                "#,
                blocked_id,
                max_id,
                limit
            )
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as!(
                BlockRow,
                r#"
                SELECT id, blocker_id, blocked_id, created_at
                FROM blocks
                WHERE blocked_id = $1
                ORDER BY id DESC
                LIMIT $2
                "#,
                blocked_id,
                limit
            )
            .fetch_all(pool)
            .await?
        };

        let blocks: Vec<Block> = block_rows
            .into_iter()
            .map(|row| Block {
                id: row.id,
                blocker_id: row.blocker_id,
                blocked_id: row.blocked_id,
                created_at: DateTime::from_naive_utc_and_offset(
                    row.created_at.expect("created_at should not be null"),
                    Utc,
                ),
            })
            .collect();

        debug!(
            "Retrieved {} blocks for blocked user {}",
            blocks.len(),
            blocked_id
        );
        Ok(blocks)
    }
}

/// Internal struct for database rows
#[derive(sqlx::FromRow)]
struct BlockRow {
    id: i64,
    blocker_id: i64,
    blocked_id: i64,
    created_at: Option<chrono::NaiveDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_block_new() {
        let block = Block {
            id: 1,
            blocker_id: 1,
            blocked_id: 2,
            created_at: Utc::now(),
        };
        assert_eq!(block.blocker_id, 1);
        assert_eq!(block.blocked_id, 2);
    }

    #[tokio::test]
    async fn test_block_operations() {
        // This would require a test database setup
        // For now, just test the structure
        let block = Block {
            id: 1,
            blocker_id: 1,
            blocked_id: 2,
            created_at: Utc::now(),
        };
        assert_eq!(block.blocker_id, 1);
    }
}
