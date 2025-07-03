//!
//! Status Pins module for Rustodon
//!
//! This module provides functionality for pinning and unpinning statuses for user accounts.
//! It handles creating, removing, and querying status pins with proper validation
//! and database operations.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_status_pins::StatusPin;
//!
//! // Pin a status
//! let pin = StatusPin::pin(&pool, account_id, status_id).await?;
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

/// Custom error type for status pins module
#[derive(Error, Debug)]
pub enum StatusPinError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Pin not found")]
    PinNotFound,
    #[error("User not found: {0}")]
    UserNotFound(i64),
    #[error("Status not found: {0}")]
    StatusNotFound(i64),
    #[error("Already pinned")]
    AlreadyPinned,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// StatusPin data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPin {
    /// Unique identifier for the pin
    pub id: i64,
    /// ID of the user who pinned the status
    pub account_id: i64,
    /// ID of the status being pinned
    pub status_id: i64,
    /// When the pin was created
    pub created_at: DateTime<Utc>,
}

impl StatusPin {
    /// Pins a status for a user
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the user
    /// * `status_id` - ID of the status to pin
    ///
    /// # Returns
    ///
    /// Result containing the created pin or an error
    pub async fn pin(
        pool: &PgPool,
        account_id: i64,
        status_id: i64,
    ) -> Result<Self, StatusPinError> {
        trace!("Pinning status {} for account {}", status_id, account_id);

        // Check if both user and status exist
        let user_exists = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM users WHERE id = $1"#,
            account_id
        )
        .fetch_one(pool)
        .await?
        .count;
        if user_exists == Some(0) {
            return Err(StatusPinError::UserNotFound(account_id));
        }
        let status_exists = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM statuses WHERE id = $1"#,
            status_id
        )
        .fetch_one(pool)
        .await?
        .count;
        if status_exists == Some(0) {
            return Err(StatusPinError::StatusNotFound(status_id));
        }
        // Check if already pinned
        let already_pinned = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM status_pins WHERE account_id = $1 AND status_id = $2"#,
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?
        .count;
        if already_pinned > Some(0) {
            return Err(StatusPinError::AlreadyPinned);
        }
        // Insert pin
        let pin_row = sqlx::query_as!(
            StatusPinRow,
            r#"
            INSERT INTO status_pins (account_id, status_id)
            VALUES ($1, $2)
            RETURNING id, account_id, status_id, created_at
            "#,
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?;
        let pin = StatusPin {
            id: pin_row.id,
            account_id: pin_row.account_id,
            status_id: pin_row.status_id,
            created_at: DateTime::from_naive_utc_and_offset(
                match pin_row.created_at {
                    Some(dt) => dt,
                    None => panic!("created_at should not be null"),
                },
                Utc,
            ),
        };
        info!(
            "Pinned status {} for account {} as pin {}",
            status_id, account_id, pin.id
        );
        Ok(pin)
    }

    /// Unpins a status for a user
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the user
    /// * `status_id` - ID of the status to unpin
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn unpin(
        pool: &PgPool,
        account_id: i64,
        status_id: i64,
    ) -> Result<(), StatusPinError> {
        trace!("Unpinning status {} for account {}", status_id, account_id);
        let result = sqlx::query!(
            r#"
            DELETE FROM status_pins
            WHERE account_id = $1 AND status_id = $2
            "#,
            account_id,
            status_id
        )
        .execute(pool)
        .await?;
        if result.rows_affected() == 0 {
            return Err(StatusPinError::PinNotFound);
        }
        info!("Unpinned status {} for account {}", status_id, account_id);
        Ok(())
    }

    /// Checks if a status is pinned by a user
    pub async fn is_pinned(
        pool: &PgPool,
        account_id: i64,
        status_id: i64,
    ) -> Result<bool, StatusPinError> {
        trace!(
            "Checking if status {} is pinned by account {}",
            status_id,
            account_id
        );
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM status_pins
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

    /// Gets all pinned statuses for a user
    pub async fn get_pins_by_account(
        pool: &PgPool,
        account_id: i64,
        limit: Option<i64>,
    ) -> Result<Vec<Self>, StatusPinError> {
        trace!(
            "Getting pins for account {} with limit {:?}",
            account_id,
            limit
        );
        let limit = limit.unwrap_or(20).min(40);
        let pin_rows = sqlx::query_as!(
            StatusPinRow,
            r#"
            SELECT id, account_id, status_id, created_at
            FROM status_pins
            WHERE account_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            account_id,
            limit
        )
        .fetch_all(pool)
        .await?;
        let pins: Vec<StatusPin> = pin_rows
            .into_iter()
            .map(|row| StatusPin {
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
        debug!("Retrieved {} pins for account {}", pins.len(), account_id);
        Ok(pins)
    }
}

/// Internal struct for database rows
#[derive(sqlx::FromRow)]
struct StatusPinRow {
    id: i64,
    account_id: i64,
    status_id: i64,
    created_at: Option<chrono::NaiveDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_status_pin_struct() {
        let pin = StatusPin {
            id: 1,
            account_id: 1,
            status_id: 2,
            created_at: Utc::now(),
        };
        assert_eq!(pin.account_id, 1);
        assert_eq!(pin.status_id, 2);
    }
    // Note: Full async DB tests would require a test database setup
}
