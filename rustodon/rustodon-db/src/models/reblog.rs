//! Reblog model for Rustodon
//!
//! This module defines the Reblog model and its database operations.
//! It handles user reblogging relationships with statuses.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use crate::error::DbError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info, trace};

/// Reblog model representing a user reblogging a status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reblog {
    pub id: i64,
    pub account_id: i64,
    pub status_id: i64,
    pub created_at: Option<NaiveDateTime>,
}

impl Reblog {
    /// Get all reblogs
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        trace!("Fetching all reblogs");
        let reblogs = sqlx::query_as!(
            Reblog,
            "SELECT id, account_id, status_id, created_at FROM reblogs ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} reblogs", reblogs.len());
        Ok(reblogs)
    }

    /// Get reblogs by account
    pub async fn get_by_account(pool: &PgPool, account_id: i64) -> Result<Vec<Self>, DbError> {
        trace!("Fetching reblogs for account: {}", account_id);
        let reblogs = sqlx::query_as!(
            Reblog,
            "SELECT id, account_id, status_id, created_at FROM reblogs WHERE account_id = $1 ORDER BY created_at DESC",
            account_id
        )
        .fetch_all(pool)
        .await?;

        info!(
            "Fetched {} reblogs for account: {}",
            reblogs.len(),
            account_id
        );
        Ok(reblogs)
    }

    /// Create a new reblog
    pub async fn create(pool: &PgPool, account_id: i64, status_id: i64) -> Result<Self, DbError> {
        trace!(
            "Creating reblog: {} reblogs status {}",
            account_id,
            status_id
        );
        let reblog = sqlx::query_as!(
            Reblog,
            "INSERT INTO reblogs (account_id, status_id) VALUES ($1, $2) RETURNING id, account_id, status_id, created_at",
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?;

        info!(
            "Created reblog: {} reblogs status {}",
            account_id, status_id
        );
        Ok(reblog)
    }

    /// Remove a reblog
    pub async fn delete(pool: &PgPool, account_id: i64, status_id: i64) -> Result<bool, DbError> {
        trace!(
            "Removing reblog: {} unreblogs status {}",
            account_id,
            status_id
        );
        let result = sqlx::query!(
            "DELETE FROM reblogs WHERE account_id = $1 AND status_id = $2",
            account_id,
            status_id
        )
        .execute(pool)
        .await?;

        let removed = result.rows_affected() > 0;
        if removed {
            info!(
                "Removed reblog: {} unreblogs status {}",
                account_id, status_id
            );
        } else {
            debug!(
                "Reblog not found for removal: {} -> {}",
                account_id, status_id
            );
        }
        Ok(removed)
    }

    /// Check if a reblog exists
    pub async fn exists(pool: &PgPool, account_id: i64, status_id: i64) -> Result<bool, DbError> {
        trace!("Checking if reblog exists: {} -> {}", account_id, status_id);
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM reblogs WHERE account_id = $1 AND status_id = $2",
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?;

        let exists = count.count.unwrap_or(0) > 0;
        debug!(
            "Reblog exists: {} -> {} = {}",
            account_id, status_id, exists
        );
        Ok(exists)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reblog_operations() {
        // This is a basic test structure
        // In a real implementation, you would set up a test database
        // and test the actual CRUD operations
        let reblog = Reblog {
            id: 1,
            account_id: 1,
            status_id: 123,
            created_at: None,
        };
        assert_eq!(reblog.account_id, 1);
        assert_eq!(reblog.status_id, 123);
    }
}
