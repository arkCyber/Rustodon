//! Favourites module for Rustodon
//!
//! This module provides favourite functionality for the Rustodon server.
//! It handles favouriting and unfavouriting statuses with proper
//! database operations and validation.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_favourites::Favourite;
//!
//! let favourite = Favourite::create(&pool, user_id, status_id).await?;
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

/// Custom error type for favourites module
#[derive(Error, Debug)]
pub enum FavouritesError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Favourite not found")]
    FavouriteNotFound,
    #[error("Status not found: {0}")]
    StatusNotFound(i64),
    #[error("User not found: {0}")]
    UserNotFound(i64),
    #[error("Already favourited")]
    AlreadyFavourited,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Favourite data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favourite {
    /// Unique identifier for the favourite
    pub id: i64,
    /// ID of the account that favourited
    pub account_id: i64,
    /// ID of the status that was favourited
    pub status_id: i64,
    /// When the favourite was created
    pub created_at: DateTime<Utc>,
}

impl Favourite {
    /// Creates a new favourite
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the account favouriting
    /// * `status_id` - ID of the status being favourited
    ///
    /// # Returns
    ///
    /// Result containing the created favourite or an error
    pub async fn create(
        pool: &PgPool,
        account_id: i64,
        status_id: i64,
    ) -> Result<Self, FavouritesError> {
        trace!(
            "Creating favourite for account {} on status {}",
            account_id,
            status_id
        );

        // Check if status exists
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

        if status_exists.unwrap_or(0) == 0 {
            return Err(FavouritesError::StatusNotFound(status_id));
        }

        // Check if user exists
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

        if user_exists.unwrap_or(0) == 0 {
            return Err(FavouritesError::UserNotFound(account_id));
        }

        // Check if already favourited
        let existing_favourite = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM favourites
            WHERE account_id = $1 AND status_id = $2
            "#,
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if existing_favourite.unwrap_or(0) > 0 {
            return Err(FavouritesError::AlreadyFavourited);
        }

        // Insert favourite
        let favourite_row = sqlx::query!(
            r#"
            INSERT INTO favourites (account_id, status_id)
            VALUES ($1, $2)
            RETURNING id, account_id, status_id, created_at
            "#,
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?;

        let favourite = Favourite {
            id: favourite_row.id,
            account_id: favourite_row.account_id,
            status_id: favourite_row.status_id,
            created_at: DateTime::from_naive_utc_and_offset(favourite_row.created_at, Utc),
        };

        info!(
            "Created favourite with id: {} for account {} on status {}",
            favourite.id, account_id, status_id
        );
        Ok(favourite)
    }

    /// Deletes a favourite
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the account
    /// * `status_id` - ID of the status
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn delete(
        pool: &PgPool,
        account_id: i64,
        status_id: i64,
    ) -> Result<(), FavouritesError> {
        trace!(
            "Deleting favourite for account {} on status {}",
            account_id,
            status_id
        );

        let result = sqlx::query!(
            r#"
            DELETE FROM favourites
            WHERE account_id = $1 AND status_id = $2
            "#,
            account_id,
            status_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(FavouritesError::FavouriteNotFound);
        }

        info!(
            "Deleted favourite for account {} on status {}",
            account_id, status_id
        );
        Ok(())
    }

    /// Checks if a user has favourited a status
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the account
    /// * `status_id` - ID of the status
    ///
    /// # Returns
    ///
    /// Result containing true if favourited, false otherwise
    pub async fn exists(
        pool: &PgPool,
        account_id: i64,
        status_id: i64,
    ) -> Result<bool, FavouritesError> {
        trace!(
            "Checking if account {} has favourited status {}",
            account_id,
            status_id
        );

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM favourites
            WHERE account_id = $1 AND status_id = $2
            "#,
            account_id,
            status_id
        )
        .fetch_one(pool)
        .await?
        .count;

        Ok(count.unwrap_or(0) > 0)
    }

    /// Gets all favourites for a user
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the account
    /// * `limit` - Maximum number of favourites to return
    /// * `since_id` - Return favourites after this ID
    /// * `max_id` - Return favourites before this ID
    ///
    /// # Returns
    ///
    /// Result containing the list of favourites or an error
    pub async fn get_by_account(
        pool: &PgPool,
        account_id: i64,
        limit: Option<i64>,
        since_id: Option<i64>,
        max_id: Option<i64>,
    ) -> Result<Vec<Self>, FavouritesError> {
        trace!(
            "Getting favourites for account {} with limit {:?}",
            account_id,
            limit
        );

        let limit = limit.unwrap_or(20).min(40);

        if let Some(since_id) = since_id {
            let favourite_rows = sqlx::query!(
                r#"
                SELECT id, account_id, status_id, created_at
                FROM favourites
                WHERE account_id = $1 AND id > $2
                ORDER BY id ASC
                LIMIT $3
                "#,
                account_id,
                since_id,
                limit
            )
            .fetch_all(pool)
            .await?;

            let favourites: Vec<Favourite> = favourite_rows
                .into_iter()
                .map(|row| Favourite {
                    id: row.id,
                    account_id: row.account_id,
                    status_id: row.status_id,
                    created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                })
                .collect();

            debug!(
                "Retrieved {} favourites for account {}",
                favourites.len(),
                account_id
            );
            return Ok(favourites);
        }

        if let Some(max_id) = max_id {
            let favourite_rows = sqlx::query!(
                r#"
                SELECT id, account_id, status_id, created_at
                FROM favourites
                WHERE account_id = $1 AND id < $2
                ORDER BY id DESC
                LIMIT $3
                "#,
                account_id,
                max_id,
                limit
            )
            .fetch_all(pool)
            .await?;

            let favourites: Vec<Favourite> = favourite_rows
                .into_iter()
                .map(|row| Favourite {
                    id: row.id,
                    account_id: row.account_id,
                    status_id: row.status_id,
                    created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                })
                .collect();

            debug!(
                "Retrieved {} favourites for account {}",
                favourites.len(),
                account_id
            );
            return Ok(favourites);
        }

        let favourite_rows = sqlx::query!(
            r#"
            SELECT id, account_id, status_id, created_at
            FROM favourites
            WHERE account_id = $1
            ORDER BY id DESC
            LIMIT $2
            "#,
            account_id,
            limit
        )
        .fetch_all(pool)
        .await?;

        let favourites: Vec<Favourite> = favourite_rows
            .into_iter()
            .map(|row| Favourite {
                id: row.id,
                account_id: row.account_id,
                status_id: row.status_id,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
            })
            .collect();

        debug!(
            "Retrieved {} favourites for account {}",
            favourites.len(),
            account_id
        );
        Ok(favourites)
    }

    /// Gets all favourites for a status
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `status_id` - ID of the status
    /// * `limit` - Maximum number of favourites to return
    /// * `since_id` - Return favourites after this ID
    /// * `max_id` - Return favourites before this ID
    ///
    /// # Returns
    ///
    /// Result containing the list of favourites or an error
    pub async fn get_by_status(
        pool: &PgPool,
        status_id: i64,
        limit: Option<i64>,
        since_id: Option<i64>,
        max_id: Option<i64>,
    ) -> Result<Vec<Self>, FavouritesError> {
        trace!(
            "Getting favourites for status {} with limit {:?}",
            status_id,
            limit
        );

        let limit = limit.unwrap_or(20).min(40);

        if let Some(since_id) = since_id {
            let favourite_rows = sqlx::query!(
                r#"
                SELECT id, account_id, status_id, created_at
                FROM favourites
                WHERE status_id = $1 AND id > $2
                ORDER BY id ASC
                LIMIT $3
                "#,
                status_id,
                since_id,
                limit
            )
            .fetch_all(pool)
            .await?;

            let favourites: Vec<Favourite> = favourite_rows
                .into_iter()
                .map(|row| Favourite {
                    id: row.id,
                    account_id: row.account_id,
                    status_id: row.status_id,
                    created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                })
                .collect();

            debug!(
                "Retrieved {} favourites for status {}",
                favourites.len(),
                status_id
            );
            return Ok(favourites);
        }

        if let Some(max_id) = max_id {
            let favourite_rows = sqlx::query!(
                r#"
                SELECT id, account_id, status_id, created_at
                FROM favourites
                WHERE status_id = $1 AND id < $2
                ORDER BY id DESC
                LIMIT $3
                "#,
                status_id,
                max_id,
                limit
            )
            .fetch_all(pool)
            .await?;

            let favourites: Vec<Favourite> = favourite_rows
                .into_iter()
                .map(|row| Favourite {
                    id: row.id,
                    account_id: row.account_id,
                    status_id: row.status_id,
                    created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                })
                .collect();

            debug!(
                "Retrieved {} favourites for status {}",
                favourites.len(),
                status_id
            );
            return Ok(favourites);
        }

        let favourite_rows = sqlx::query!(
            r#"
            SELECT id, account_id, status_id, created_at
            FROM favourites
            WHERE status_id = $1
            ORDER BY id DESC
            LIMIT $2
            "#,
            status_id,
            limit
        )
        .fetch_all(pool)
        .await?;

        let favourites: Vec<Favourite> = favourite_rows
            .into_iter()
            .map(|row| Favourite {
                id: row.id,
                account_id: row.account_id,
                status_id: row.status_id,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
            })
            .collect();

        debug!(
            "Retrieved {} favourites for status {}",
            favourites.len(),
            status_id
        );
        Ok(favourites)
    }

    /// Gets the count of favourites for a status
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `status_id` - ID of the status
    ///
    /// # Returns
    ///
    /// Result containing the count or an error
    pub async fn count_by_status(pool: &PgPool, status_id: i64) -> Result<i64, FavouritesError> {
        trace!("Getting favourite count for status {}", status_id);

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM favourites
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_favourite_create_and_delete() {
        // This would require a test database setup
        // For now, just test the struct creation
        let favourite = Favourite {
            id: 1,
            account_id: 1,
            status_id: 1,
            created_at: Utc::now(),
        };

        assert_eq!(favourite.account_id, 1);
        assert_eq!(favourite.status_id, 1);
    }
}
