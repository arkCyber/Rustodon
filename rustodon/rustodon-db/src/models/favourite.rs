//! Favourite model for Rustodon
//!
//! This module defines the Favourite model and its database operations.
//! It handles user favouriting relationships with statuses.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use crate::error::DbError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info, trace};

/// Favourite model representing a user favouriting a status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favourite {
    pub id: i64,
    pub account_id: i64,
    pub status_id: i64,
    pub created_at: Option<NaiveDateTime>,
}

impl Favourite {
    /// Get all favourites
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        trace!("Fetching all favourites");
        let favourites = sqlx::query_as!(
            Favourite,
            "SELECT id, account_id, status_id, created_at FROM favourites ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} favourites", favourites.len());
        Ok(favourites)
    }

    /// Get favourites by account
    pub async fn get_by_account(pool: &PgPool, account_id: i64) -> Result<Vec<Self>, DbError> {
        trace!("Fetching favourites for account: {}", account_id);
        let favourites = sqlx::query_as!(
            Favourite,
            "SELECT id, account_id, status_id, created_at FROM favourites WHERE account_id = $1 ORDER BY created_at DESC",
            account_id
        )
        .fetch_all(pool)
        .await?;

        info!(
            "Fetched {} favourites for account: {}",
            favourites.len(),
            account_id
        );
        Ok(favourites)
    }

    /// Create a new favourite
    pub async fn create(pool: &PgPool, account_id: i64, status_id: i64) -> Result<Self, DbError> {
        trace!(
            "Creating favourite: {} favours status {}",
            account_id,
            status_id
        );
        let favourite = sqlx::query_as!(
            Favourite,
            "INSERT INTO favourites (account_id, status_id) VALUES ($1, $2) RETURNING id, account_id, status_id, created_at",
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?;

        info!(
            "Created favourite: {} favours status {}",
            account_id, status_id
        );
        Ok(favourite)
    }

    /// Remove a favourite
    pub async fn delete(pool: &PgPool, account_id: i64, status_id: i64) -> Result<bool, DbError> {
        trace!(
            "Removing favourite: {} unfavours status {}",
            account_id,
            status_id
        );
        let result = sqlx::query!(
            "DELETE FROM favourites WHERE account_id = $1 AND status_id = $2",
            account_id,
            status_id
        )
        .execute(pool)
        .await?;

        let removed = result.rows_affected() > 0;
        if removed {
            info!(
                "Removed favourite: {} unfavours status {}",
                account_id, status_id
            );
        } else {
            debug!(
                "Favourite not found for removal: {} -> {}",
                account_id, status_id
            );
        }
        Ok(removed)
    }

    /// Check if a favourite exists
    pub async fn exists(pool: &PgPool, account_id: i64, status_id: i64) -> Result<bool, DbError> {
        trace!(
            "Checking if favourite exists: {} -> {}",
            account_id,
            status_id
        );
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM favourites WHERE account_id = $1 AND status_id = $2",
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?;

        let exists = count.count.unwrap_or(0) > 0;
        debug!(
            "Favourite exists: {} -> {} = {}",
            account_id, status_id, exists
        );
        Ok(exists)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_favourite_operations() {
        // This is a basic test structure
        // In a real implementation, you would set up a test database
        // and test the actual CRUD operations
        let favourite = Favourite {
            id: 1,
            account_id: 1,
            status_id: 1,
            created_at: None,
        };
        assert_eq!(favourite.account_id, 1);
        assert_eq!(favourite.status_id, 1);
    }
}
