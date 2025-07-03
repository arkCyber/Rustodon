//! Reblogs module for Rustodon
//!
//! This module provides functionality for managing reblogs (reposts) in the Rustodon server.
//! It handles creating, removing, and querying reblogs with proper validation
//! and database operations.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_reblogs::Reblog;
//!
//! let reblog = Reblog::create(&pool, account_id, status_id).await?;
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

/// Custom error type for reblogs module
#[derive(Error, Debug)]
pub enum ReblogError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Reblog not found")]
    ReblogNotFound,
    #[error("User not found: {0}")]
    UserNotFound(i64),
    #[error("Status not found: {0}")]
    StatusNotFound(i64),
    #[error("Cannot reblog your own status")]
    CannotReblogOwnStatus,
    #[error("Already reblogged")]
    AlreadyReblogged,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Reblog data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reblog {
    /// Unique identifier for the reblog
    pub id: i64,
    /// ID of the user doing the reblog
    pub account_id: i64,
    /// ID of the status being reblogged
    pub status_id: i64,
    /// When the reblog was created
    pub created_at: DateTime<Utc>,
}

impl Reblog {
    /// Creates a new reblog
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the user doing the reblog
    /// * `status_id` - ID of the status being reblogged
    ///
    /// # Returns
    ///
    /// Result containing the created reblog or an error
    pub async fn create(
        pool: &PgPool,
        account_id: i64,
        status_id: i64,
    ) -> Result<Self, ReblogError> {
        trace!(
            "Creating reblog from account {} for status {}",
            account_id,
            status_id
        );

        // Check if both user and status exist
        let user_exists = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM users
            WHERE id = $1
            "#,
            account_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if user_exists == Some(0) {
            return Err(ReblogError::UserNotFound(account_id));
        }

        let status_exists = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM statuses
            WHERE id = $1
            "#,
            status_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if status_exists == Some(0) {
            return Err(ReblogError::StatusNotFound(status_id));
        }

        // Check if already reblogged
        let existing_reblog = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM reblogs
            WHERE account_id = $1 AND status_id = $2
            "#,
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if existing_reblog > Some(0) {
            return Err(ReblogError::AlreadyReblogged);
        }

        // Insert reblog
        let reblog_row = sqlx::query_as!(
            ReblogRow,
            r#"
            INSERT INTO reblogs (account_id, status_id)
            VALUES ($1, $2)
            RETURNING id, account_id, status_id, created_at
            "#,
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?;

        let reblog = Reblog {
            id: reblog_row.id,
            account_id: reblog_row.account_id,
            status_id: reblog_row.status_id,
            created_at: DateTime::from_naive_utc_and_offset(
                match reblog_row.created_at {
                    Some(dt) => dt,
                    None => panic!("created_at should not be null"),
                },
                Utc,
            ),
        };

        info!(
            "Created reblog with id: {} from account {} for status {}",
            reblog.id, account_id, status_id
        );
        Ok(reblog)
    }

    /// Removes a reblog
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the user who did the reblog
    /// * `status_id` - ID of the status that was reblogged
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn remove(pool: &PgPool, account_id: i64, status_id: i64) -> Result<(), ReblogError> {
        trace!(
            "Removing reblog from account {} for status {}",
            account_id,
            status_id
        );

        let result = sqlx::query!(
            r#"
            DELETE FROM reblogs
            WHERE account_id = $1 AND status_id = $2
            "#,
            account_id,
            status_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ReblogError::ReblogNotFound);
        }

        info!(
            "Removed reblog from account {} for status {}",
            account_id, status_id
        );
        Ok(())
    }

    /// Checks if a reblog exists
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the user who did the reblog
    /// * `status_id` - ID of the status that was reblogged
    ///
    /// # Returns
    ///
    /// Result containing true if reblogged, false otherwise
    pub async fn exists(
        pool: &PgPool,
        account_id: i64,
        status_id: i64,
    ) -> Result<bool, ReblogError> {
        trace!(
            "Checking if reblog exists from account {} for status {}",
            account_id,
            status_id
        );

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM reblogs
            WHERE account_id = $1 AND status_id = $2
            "#,
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?
        .count;

        Ok(count > Some(0))
    }

    /// Gets all reblogs for a user
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the user who did the reblogs
    /// * `limit` - Maximum number of reblogs to return
    /// * `since_id` - Return reblogs after this ID
    /// * `max_id` - Return reblogs before this ID
    ///
    /// # Returns
    ///
    /// Result containing the list of reblogs or an error
    pub async fn get_by_account(
        pool: &PgPool,
        account_id: i64,
        limit: Option<i64>,
        since_id: Option<i64>,
        max_id: Option<i64>,
    ) -> Result<Vec<Self>, ReblogError> {
        trace!(
            "Getting reblogs for account {} with limit {:?}",
            account_id,
            limit
        );

        let limit = limit.unwrap_or(20).min(40);

        let reblog_rows = if let Some(since_id) = since_id {
            sqlx::query_as!(
                ReblogRow,
                r#"
                SELECT id, account_id, status_id, created_at
                FROM reblogs
                WHERE account_id = $1 AND id > $2
                ORDER BY id ASC
                LIMIT $3
                "#,
                account_id,
                since_id,
                limit
            )
            .fetch_all(pool)
            .await?
        } else if let Some(max_id) = max_id {
            sqlx::query_as!(
                ReblogRow,
                r#"
                SELECT id, account_id, status_id, created_at
                FROM reblogs
                WHERE account_id = $1 AND id < $2
                ORDER BY id DESC
                LIMIT $3
                "#,
                account_id,
                max_id,
                limit
            )
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as!(
                ReblogRow,
                r#"
                SELECT id, account_id, status_id, created_at
                FROM reblogs
                WHERE account_id = $1
                ORDER BY id DESC
                LIMIT $2
                "#,
                account_id,
                limit
            )
            .fetch_all(pool)
            .await?
        };

        let reblogs: Vec<Reblog> = reblog_rows
            .into_iter()
            .map(|row| Reblog {
                id: row.id,
                account_id: row.account_id,
                status_id: row.status_id,
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
            "Retrieved {} reblogs for account {}",
            reblogs.len(),
            account_id
        );
        Ok(reblogs)
    }

    /// Gets all reblogs for a status
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `status_id` - ID of the status that was reblogged
    /// * `limit` - Maximum number of reblogs to return
    /// * `since_id` - Return reblogs after this ID
    /// * `max_id` - Return reblogs before this ID
    ///
    /// # Returns
    ///
    /// Result containing the list of reblogs or an error
    pub async fn get_by_status(
        pool: &PgPool,
        status_id: i64,
        limit: Option<i64>,
        since_id: Option<i64>,
        max_id: Option<i64>,
    ) -> Result<Vec<Self>, ReblogError> {
        trace!(
            "Getting reblogs for status {} with limit {:?}",
            status_id,
            limit
        );

        let limit = limit.unwrap_or(20).min(40);

        let reblog_rows = if let Some(since_id) = since_id {
            sqlx::query_as!(
                ReblogRow,
                r#"
                SELECT id, account_id, status_id, created_at
                FROM reblogs
                WHERE status_id = $1 AND id > $2
                ORDER BY id ASC
                LIMIT $3
                "#,
                status_id,
                since_id,
                limit
            )
            .fetch_all(pool)
            .await?
        } else if let Some(max_id) = max_id {
            sqlx::query_as!(
                ReblogRow,
                r#"
                SELECT id, account_id, status_id, created_at
                FROM reblogs
                WHERE status_id = $1 AND id < $2
                ORDER BY id DESC
                LIMIT $3
                "#,
                status_id,
                max_id,
                limit
            )
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as!(
                ReblogRow,
                r#"
                SELECT id, account_id, status_id, created_at
                FROM reblogs
                WHERE status_id = $1
                ORDER BY id DESC
                LIMIT $2
                "#,
                status_id,
                limit
            )
            .fetch_all(pool)
            .await?
        };

        let reblogs: Vec<Reblog> = reblog_rows
            .into_iter()
            .map(|row| Reblog {
                id: row.id,
                account_id: row.account_id,
                status_id: row.status_id,
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
            "Retrieved {} reblogs for status {}",
            reblogs.len(),
            status_id
        );
        Ok(reblogs)
    }

    /// Gets the count of reblogs for a status
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `status_id` - ID of the status
    ///
    /// # Returns
    ///
    /// Result containing the count of reblogs
    pub async fn get_count_for_status(pool: &PgPool, status_id: i64) -> Result<i64, ReblogError> {
        trace!("Getting reblog count for status {}", status_id);

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM reblogs
            WHERE status_id = $1
            "#,
            status_id
        )
        .fetch_one(pool)
        .await?
        .count;

        Ok(count.unwrap_or(0))
    }
}

/// Internal struct for database rows
#[derive(sqlx::FromRow)]
struct ReblogRow {
    id: i64,
    account_id: i64,
    status_id: i64,
    created_at: Option<chrono::NaiveDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reblog_new() {
        let reblog = Reblog {
            id: 1,
            account_id: 1,
            status_id: 2,
            created_at: Utc::now(),
        };
        assert_eq!(reblog.account_id, 1);
        assert_eq!(reblog.status_id, 2);
    }

    #[tokio::test]
    async fn test_reblog_operations() {
        // This would require a test database setup
        // For now, just test the structure
        let reblog = Reblog {
            id: 1,
            account_id: 1,
            status_id: 2,
            created_at: Utc::now(),
        };
        assert_eq!(reblog.account_id, 1);
    }
}
