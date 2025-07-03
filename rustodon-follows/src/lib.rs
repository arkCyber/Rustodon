//! Follows module for Rustodon
//!
//! This module provides follow functionality for the Rustodon server.
//! It handles following and unfollowing accounts with proper
//! database operations and validation.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_follows::Follow;
//!
//! let follow = Follow::create(&pool, follower_id, followed_id).await?;
//! ```
//!
//! # Dependencies
//!
//! - `rustodon_core`: Core types and traits
//! - `rustodon_db`: Database operations
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
use tracing::{debug, error, info, trace};

/// Custom error type for follows module
#[derive(Error, Debug)]
pub enum FollowsError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Follow not found")]
    FollowNotFound,
    #[error("Follower not found: {0}")]
    FollowerNotFound(i64),
    #[error("Followed account not found: {0}")]
    FollowedNotFound(i64),
    #[error("Already following")]
    AlreadyFollowing,
    #[error("Cannot follow yourself")]
    CannotFollowSelf,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Follow data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Follow {
    /// Unique identifier for the follow
    pub id: i64,
    /// ID of the account that is following
    pub follower_id: i64,
    /// ID of the account being followed
    pub followed_id: i64,
    /// Whether to show reblogs from the followed account
    pub show_reblogs: bool,
    /// Whether to notify when the followed account posts
    pub notify: bool,
    /// When the follow was created
    pub created_at: DateTime<Utc>,
    /// When the follow was last updated
    pub updated_at: DateTime<Utc>,
}

impl Follow {
    /// Creates a new follow relationship
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `follower_id` - ID of the account following
    /// * `followed_id` - ID of the account being followed
    ///
    /// # Returns
    ///
    /// Result containing the created follow or an error
    pub async fn create(
        pool: &PgPool,
        follower_id: i64,
        followed_id: i64,
    ) -> Result<Self, FollowsError> {
        trace!("Creating follow from {} to {}", follower_id, followed_id);

        // Check if trying to follow self
        if follower_id == followed_id {
            return Err(FollowsError::CannotFollowSelf);
        }

        // Check if follower exists
        let follower_exists = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM users
            WHERE id = $1
            "#,
            follower_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if follower_exists.unwrap_or(0) == 0 {
            return Err(FollowsError::FollowerNotFound(follower_id));
        }

        // Check if followed account exists
        let followed_exists = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM users
            WHERE id = $1
            "#,
            followed_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if followed_exists.unwrap_or(0) == 0 {
            return Err(FollowsError::FollowedNotFound(followed_id));
        }

        // Check if already following
        let existing_follow = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM follows
            WHERE follower_id = $1 AND followed_id = $2
            "#,
            follower_id,
            followed_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if existing_follow.unwrap_or(0) > 0 {
            return Err(FollowsError::AlreadyFollowing);
        }

        // Insert follow
        let follow_row = sqlx::query!(
            r#"
            INSERT INTO follows (follower_id, followed_id, show_reblogs, notify)
            VALUES ($1, $2, true, false)
            RETURNING id, follower_id, followed_id, show_reblogs, notify, created_at, updated_at
            "#,
            follower_id,
            followed_id
        )
        .fetch_one(pool)
        .await?;

        let follow = Follow {
            id: follow_row.id,
            follower_id: follow_row.follower_id,
            followed_id: follow_row.followed_id,
            show_reblogs: follow_row.show_reblogs.unwrap_or(true),
            notify: follow_row.notify.unwrap_or(false),
            created_at: DateTime::from_naive_utc_and_offset(follow_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(follow_row.updated_at, Utc),
        };

        info!(
            "Created follow with id: {} from {} to {}",
            follow.id, follower_id, followed_id
        );
        Ok(follow)
    }

    /// Deletes a follow relationship
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `follower_id` - ID of the follower
    /// * `followed_id` - ID of the followed account
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn delete(
        pool: &PgPool,
        follower_id: i64,
        followed_id: i64,
    ) -> Result<(), FollowsError> {
        trace!("Deleting follow from {} to {}", follower_id, followed_id);

        let result = sqlx::query!(
            r#"
            DELETE FROM follows
            WHERE follower_id = $1 AND followed_id = $2
            "#,
            follower_id,
            followed_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(FollowsError::FollowNotFound);
        }

        info!("Deleted follow from {} to {}", follower_id, followed_id);
        Ok(())
    }

    /// Checks if one account is following another
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `follower_id` - ID of the follower
    /// * `followed_id` - ID of the followed account
    ///
    /// # Returns
    ///
    /// Result containing true if following, false otherwise
    pub async fn exists(
        pool: &PgPool,
        follower_id: i64,
        followed_id: i64,
    ) -> Result<bool, FollowsError> {
        trace!("Checking if {} is following {}", follower_id, followed_id);

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM follows
            WHERE follower_id = $1 AND followed_id = $2
            "#,
            follower_id,
            followed_id
        )
        .fetch_one(pool)
        .await?
        .count;

        Ok(count.unwrap_or(0) > 0)
    }

    /// Gets all accounts that a user is following
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `follower_id` - ID of the follower
    /// * `limit` - Maximum number of follows to return
    /// * `since_id` - Return follows after this ID
    /// * `max_id` - Return follows before this ID
    ///
    /// # Returns
    ///
    /// Result containing the list of follows or an error
    pub async fn get_following(
        pool: &PgPool,
        follower_id: i64,
        limit: Option<i64>,
        since_id: Option<i64>,
        max_id: Option<i64>,
    ) -> Result<Vec<Self>, FollowsError> {
        trace!(
            "Getting following for account {} with limit {:?}",
            follower_id,
            limit
        );
        let limit = limit.unwrap_or(20).min(40);
        if let Some(since_id) = since_id {
            let follow_rows = sqlx::query!(
                r#"
                SELECT id, follower_id, followed_id, show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE follower_id = $1 AND id > $2
                ORDER BY id ASC
                LIMIT $3
                "#,
                follower_id,
                since_id,
                limit
            )
            .fetch_all(pool)
            .await?;
            let follows: Vec<Follow> = follow_rows
                .into_iter()
                .map(|row| Follow {
                    id: row.id,
                    follower_id: row.follower_id,
                    followed_id: row.followed_id,
                    show_reblogs: row.show_reblogs.unwrap_or(true),
                    notify: row.notify.unwrap_or(false),
                    created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                    updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
                })
                .collect();
            debug!(
                "Retrieved {} following for account {}",
                follows.len(),
                follower_id
            );
            Ok(follows)
        } else if let Some(max_id) = max_id {
            let follow_rows = sqlx::query!(
                r#"
                SELECT id, follower_id, followed_id, show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE follower_id = $1 AND id < $2
                ORDER BY id DESC
                LIMIT $3
                "#,
                follower_id,
                max_id,
                limit
            )
            .fetch_all(pool)
            .await?;
            let follows: Vec<Follow> = follow_rows
                .into_iter()
                .map(|row| Follow {
                    id: row.id,
                    follower_id: row.follower_id,
                    followed_id: row.followed_id,
                    show_reblogs: row.show_reblogs.unwrap_or(true),
                    notify: row.notify.unwrap_or(false),
                    created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                    updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
                })
                .collect();
            debug!(
                "Retrieved {} following for account {}",
                follows.len(),
                follower_id
            );
            return Ok(follows);
        } else {
            let follow_rows = sqlx::query!(
                r#"
                SELECT id, follower_id, followed_id, show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE follower_id = $1
                ORDER BY id DESC
                LIMIT $2
                "#,
                follower_id,
                limit
            )
            .fetch_all(pool)
            .await?;
            let follows: Vec<Follow> = follow_rows
                .into_iter()
                .map(|row| Follow {
                    id: row.id,
                    follower_id: row.follower_id,
                    followed_id: row.followed_id,
                    show_reblogs: row.show_reblogs.unwrap_or(true),
                    notify: row.notify.unwrap_or(false),
                    created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                    updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
                })
                .collect();
            debug!(
                "Retrieved {} following for account {}",
                follows.len(),
                follower_id
            );
            return Ok(follows);
        }
    }

    /// Gets all accounts that are following a user
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `followed_id` - ID of the followed account
    /// * `limit` - Maximum number of follows to return
    /// * `since_id` - Return follows after this ID
    /// * `max_id` - Return follows before this ID
    ///
    /// # Returns
    ///
    /// Result containing the list of follows or an error
    pub async fn get_followers(
        pool: &PgPool,
        followed_id: i64,
        limit: Option<i64>,
        since_id: Option<i64>,
        max_id: Option<i64>,
    ) -> Result<Vec<Self>, FollowsError> {
        trace!(
            "Getting followers for account {} with limit {:?}",
            followed_id,
            limit
        );
        let limit = limit.unwrap_or(20).min(40);
        if let Some(since_id) = since_id {
            let follow_rows = sqlx::query!(
                r#"
                SELECT id, follower_id, followed_id, show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE followed_id = $1 AND id > $2
                ORDER BY id ASC
                LIMIT $3
                "#,
                followed_id,
                since_id,
                limit
            )
            .fetch_all(pool)
            .await?;
            let follows: Vec<Follow> = follow_rows
                .into_iter()
                .map(|row| Follow {
                    id: row.id,
                    follower_id: row.follower_id,
                    followed_id: row.followed_id,
                    show_reblogs: row.show_reblogs.unwrap_or(true),
                    notify: row.notify.unwrap_or(false),
                    created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                    updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
                })
                .collect();
            debug!(
                "Retrieved {} followers for account {}",
                follows.len(),
                followed_id
            );
            Ok(follows)
        } else if let Some(max_id) = max_id {
            let follow_rows = sqlx::query!(
                r#"
                SELECT id, follower_id, followed_id, show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE followed_id = $1 AND id < $2
                ORDER BY id DESC
                LIMIT $3
                "#,
                followed_id,
                max_id,
                limit
            )
            .fetch_all(pool)
            .await?;
            let follows: Vec<Follow> = follow_rows
                .into_iter()
                .map(|row| Follow {
                    id: row.id,
                    follower_id: row.follower_id,
                    followed_id: row.followed_id,
                    show_reblogs: row.show_reblogs.unwrap_or(true),
                    notify: row.notify.unwrap_or(false),
                    created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                    updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
                })
                .collect();
            debug!(
                "Retrieved {} followers for account {}",
                follows.len(),
                followed_id
            );
            return Ok(follows);
        } else {
            let follow_rows = sqlx::query!(
                r#"
                SELECT id, follower_id, followed_id, show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE followed_id = $1
                ORDER BY id DESC
                LIMIT $2
                "#,
                followed_id,
                limit
            )
            .fetch_all(pool)
            .await?;
            let follows: Vec<Follow> = follow_rows
                .into_iter()
                .map(|row| Follow {
                    id: row.id,
                    follower_id: row.follower_id,
                    followed_id: row.followed_id,
                    show_reblogs: row.show_reblogs.unwrap_or(true),
                    notify: row.notify.unwrap_or(false),
                    created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                    updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
                })
                .collect();
            debug!(
                "Retrieved {} followers for account {}",
                follows.len(),
                followed_id
            );
            return Ok(follows);
        }
    }

    /// Gets the count of accounts a user is following
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `follower_id` - ID of the follower
    ///
    /// # Returns
    ///
    /// Result containing the count or an error
    pub async fn count_following(pool: &PgPool, follower_id: i64) -> Result<i64, FollowsError> {
        trace!("Getting following count for account {}", follower_id);

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM follows
            WHERE follower_id = $1
            "#,
            follower_id
        )
        .fetch_one(pool)
        .await?
        .count;

        Ok(count.unwrap_or(0))
    }

    /// Gets the count of accounts following a user
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `followed_id` - ID of the followed account
    ///
    /// # Returns
    ///
    /// Result containing the count or an error
    pub async fn count_followers(pool: &PgPool, followed_id: i64) -> Result<i64, FollowsError> {
        trace!("Getting followers count for account {}", followed_id);

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM follows
            WHERE followed_id = $1
            "#,
            followed_id
        )
        .fetch_one(pool)
        .await?
        .count;

        Ok(count.unwrap_or(0))
    }

    /// Updates follow settings
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `follower_id` - ID of the follower
    /// * `followed_id` - ID of the followed account
    /// * `show_reblogs` - Whether to show reblogs
    /// * `notify` - Whether to notify on posts
    ///
    /// # Returns
    ///
    /// Result containing the updated follow or an error
    pub async fn update_settings(
        pool: &PgPool,
        follower_id: i64,
        followed_id: i64,
        show_reblogs: bool,
        notify: bool,
    ) -> Result<Self, FollowsError> {
        trace!(
            "Updating follow settings from {} to {}: show_reblogs={}, notify={}",
            follower_id,
            followed_id,
            show_reblogs,
            notify
        );

        let follow_row = sqlx::query!(
            r#"
            UPDATE follows
            SET show_reblogs = $3, notify = $4, updated_at = now()
            WHERE follower_id = $1 AND followed_id = $2
            RETURNING id, follower_id, followed_id, show_reblogs, notify, created_at, updated_at
            "#,
            follower_id,
            followed_id,
            show_reblogs,
            notify
        )
        .fetch_optional(pool)
        .await?
        .ok_or(FollowsError::FollowNotFound)?;

        let follow = Follow {
            id: follow_row.id,
            follower_id: follow_row.follower_id,
            followed_id: follow_row.followed_id,
            show_reblogs: follow_row.show_reblogs.unwrap_or(true),
            notify: follow_row.notify.unwrap_or(false),
            created_at: DateTime::from_naive_utc_and_offset(follow_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(follow_row.updated_at, Utc),
        };

        info!(
            "Updated follow settings for follow {} from {} to {}",
            follow.id, follower_id, followed_id
        );
        Ok(follow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_follow_create_and_delete() {
        // This would require a test database setup
        // For now, just test the struct creation
        let follow = Follow {
            id: 1,
            follower_id: 1,
            followed_id: 2,
            show_reblogs: true,
            notify: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(follow.follower_id, 1);
        assert_eq!(follow.followed_id, 2);
        assert!(follow.show_reblogs);
        assert!(!follow.notify);
    }
}
