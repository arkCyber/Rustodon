//!
//! Tag Follows module for Rustodon
//!
//! This module provides functionality for following and unfollowing tags for user accounts.
//! It handles creating, removing, and querying tag follows with proper validation
//! and database operations.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_tag_follows::TagFollow;
//!
//! // Follow a tag
//! let follow = TagFollow::follow(&pool, account_id, tag_id).await?;
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

/// Custom error type for tag follows module
#[derive(Error, Debug)]
pub enum TagFollowError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Tag follow not found")]
    TagFollowNotFound,
    #[error("User not found: {0}")]
    UserNotFound(i64),
    #[error("Tag not found: {0}")]
    TagNotFound(i64),
    #[error("Already following tag")]
    AlreadyFollowing,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// TagFollow data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagFollow {
    /// Unique identifier for the tag follow
    pub id: i64,
    /// ID of the user who follows the tag
    pub account_id: i64,
    /// ID of the tag being followed
    pub tag_id: i64,
    /// When the follow was created
    pub created_at: DateTime<Utc>,
}

impl TagFollow {
    /// Follows a tag for a user
    pub async fn follow(
        pool: &PgPool,
        account_id: i64,
        tag_id: i64,
    ) -> Result<Self, TagFollowError> {
        trace!("Following tag {} for account {}", tag_id, account_id);
        // Check if both user and tag exist
        let user_exists = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM users WHERE id = $1"#,
            account_id
        )
        .fetch_one(pool)
        .await?
        .count;
        if user_exists == Some(0) {
            return Err(TagFollowError::UserNotFound(account_id));
        }
        let tag_exists = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM tags WHERE id = $1"#,
            tag_id
        )
        .fetch_one(pool)
        .await?
        .count;
        if tag_exists == Some(0) {
            return Err(TagFollowError::TagNotFound(tag_id));
        }
        // Check if already following
        let already_following = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM tag_follows WHERE account_id = $1 AND tag_id = $2"#,
            account_id,
            tag_id
        )
        .fetch_one(pool)
        .await?
        .count;
        if already_following > Some(0) {
            return Err(TagFollowError::AlreadyFollowing);
        }
        // Insert follow
        let follow_row = sqlx::query_as!(
            TagFollowRow,
            r#"
            INSERT INTO tag_follows (account_id, tag_id)
            VALUES ($1, $2)
            RETURNING id, account_id, tag_id, created_at
            "#,
            account_id,
            tag_id
        )
        .fetch_one(pool)
        .await?;
        let follow = TagFollow {
            id: follow_row.id,
            account_id: follow_row.account_id,
            tag_id: follow_row.tag_id,
            created_at: DateTime::from_naive_utc_and_offset(
                match follow_row.created_at {
                    Some(dt) => dt,
                    None => panic!("created_at should not be null"),
                },
                Utc,
            ),
        };
        info!(
            "Account {} followed tag {} as follow {}",
            account_id, tag_id, follow.id
        );
        Ok(follow)
    }

    /// Unfollows a tag for a user
    pub async fn unfollow(
        pool: &PgPool,
        account_id: i64,
        tag_id: i64,
    ) -> Result<(), TagFollowError> {
        trace!("Unfollowing tag {} for account {}", tag_id, account_id);
        let result = sqlx::query!(
            r#"
            DELETE FROM tag_follows
            WHERE account_id = $1 AND tag_id = $2
            "#,
            account_id,
            tag_id
        )
        .execute(pool)
        .await?;
        if result.rows_affected() == 0 {
            return Err(TagFollowError::TagFollowNotFound);
        }
        info!("Account {} unfollowed tag {}", account_id, tag_id);
        Ok(())
    }

    /// Checks if a tag is followed by a user
    pub async fn is_following(
        pool: &PgPool,
        account_id: i64,
        tag_id: i64,
    ) -> Result<bool, TagFollowError> {
        trace!(
            "Checking if tag {} is followed by account {}",
            tag_id,
            account_id
        );
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM tag_follows
            WHERE account_id = $1 AND tag_id = $2
            "#,
            account_id,
            tag_id
        )
        .fetch_one(pool)
        .await?
        .count;
        Ok(count > Some(0))
    }

    /// Gets all followed tags for a user
    pub async fn get_follows_by_account(
        pool: &PgPool,
        account_id: i64,
        limit: Option<i64>,
    ) -> Result<Vec<Self>, TagFollowError> {
        trace!(
            "Getting tag follows for account {} with limit {:?}",
            account_id,
            limit
        );
        let limit = limit.unwrap_or(20).min(40);
        let follow_rows = sqlx::query_as!(
            TagFollowRow,
            r#"
            SELECT id, account_id, tag_id, created_at
            FROM tag_follows
            WHERE account_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            account_id,
            limit
        )
        .fetch_all(pool)
        .await?;
        let follows: Vec<TagFollow> = follow_rows
            .into_iter()
            .map(|row| TagFollow {
                id: row.id,
                account_id: row.account_id,
                tag_id: row.tag_id,
                created_at: DateTime::from_naive_utc_and_offset(
                    match row.created_at {
                        Some(dt) => dt,
                        None => panic!("created_at should not be null"),
                    },
                    Utc,
                ),
            })
            .collect();
        debug!(
            "Retrieved {} tag follows for account {}",
            follows.len(),
            account_id
        );
        Ok(follows)
    }
}

/// Internal struct for database rows
#[derive(sqlx::FromRow)]
struct TagFollowRow {
    id: i64,
    account_id: i64,
    tag_id: i64,
    created_at: Option<chrono::NaiveDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_tag_follow_struct() {
        let tag_follow = TagFollow {
            id: 1,
            account_id: 1,
            tag_id: 1,
            created_at: Utc::now(),
        };
        assert_eq!(tag_follow.account_id, 1);
        assert_eq!(tag_follow.tag_id, 1);
    }
    // Note: Full async DB tests would require a test database setup
}
