//! Filters module for Rustodon
//!
//! This module provides filter functionality for the Rustodon server.
//! It handles creating, managing, and applying content filters
//! with proper database operations and validation.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_filters::{Filter, FilterKeyword};
//!
//! let filter = Filter::create(&pool, user_id, "My Filter", "home").await?;
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

/// Custom error type for filters module
#[derive(Error, Debug)]
pub enum FiltersError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Filter not found: {0}")]
    FilterNotFound(i64),
    #[error("Filter keyword not found: {0}")]
    FilterKeywordNotFound(i64),
    #[error("User not found: {0}")]
    UserNotFound(i64),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Filter data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    /// Unique identifier for the filter
    pub id: i64,
    /// ID of the account that owns the filter
    pub account_id: i64,
    /// Title of the filter
    pub title: String,
    /// Context where the filter applies (home, notifications, public, thread)
    pub context: String,
    /// When the filter expires (None if no expiration)
    pub expires_at: Option<DateTime<Utc>>,
    /// Whether to filter from home timeline
    pub filter_action: String,
    /// When the filter was created
    pub created_at: DateTime<Utc>,
    /// When the filter was last updated
    pub updated_at: DateTime<Utc>,
    /// Keywords associated with this filter
    pub keywords: Vec<FilterKeyword>,
}

/// Filter keyword data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterKeyword {
    /// Unique identifier for the filter keyword
    pub id: i64,
    /// ID of the filter this keyword belongs to
    pub filter_id: i64,
    /// The keyword to filter
    pub keyword: String,
    /// Whether to match whole words only
    pub whole_word: bool,
    /// When the keyword was created
    pub created_at: DateTime<Utc>,
}

/// Create filter request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterRequest {
    /// Title of the filter
    pub title: String,
    /// Context where the filter applies
    pub context: Vec<String>,
    /// When the filter expires (None if no expiration)
    pub expires_at: Option<DateTime<Utc>>,
    /// Filter action (warn, hide)
    pub filter_action: String,
    /// Keywords to filter
    pub keywords: Vec<String>,
}

/// Update filter request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterRequest {
    /// Title of the filter
    pub title: Option<String>,
    /// Context where the filter applies
    pub context: Option<Vec<String>>,
    /// When the filter expires (None if no expiration)
    pub expires_at: Option<DateTime<Utc>>,
    /// Filter action (warn, hide)
    pub filter_action: Option<String>,
    /// Keywords to filter
    pub keywords: Option<Vec<String>>,
}

impl Filter {
    /// Creates a new filter
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the account creating the filter
    /// * `request` - Filter creation request
    ///
    /// # Returns
    ///
    /// Result containing the created filter or an error
    pub async fn create(
        pool: &PgPool,
        account_id: i64,
        request: CreateFilterRequest,
    ) -> Result<Self, FiltersError> {
        trace!(
            "Creating filter for account {} with title: {}",
            account_id,
            request.title
        );

        // Validate request
        if request.title.trim().is_empty() {
            return Err(FiltersError::Validation(
                "Filter title cannot be empty".to_string(),
            ));
        }
        if request.title.len() > 100 {
            return Err(FiltersError::Validation(
                "Filter title cannot exceed 100 characters".to_string(),
            ));
        }
        if request.context.is_empty() {
            return Err(FiltersError::Validation(
                "Filter must have at least one context".to_string(),
            ));
        }
        if request.keywords.is_empty() {
            return Err(FiltersError::Validation(
                "Filter must have at least one keyword".to_string(),
            ));
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

        if user_exists == Some(0) {
            return Err(FiltersError::UserNotFound(account_id));
        }

        // Start transaction
        let mut tx = pool.begin().await?;

        // Insert filter
        let filter_row = sqlx::query!(
            r#"
            INSERT INTO filters (account_id, title, context, expires_at, filter_action)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, account_id, title, context, expires_at, filter_action, created_at, updated_at
            "#,
            account_id,
            request.title,
            request.context.as_slice(),
            request.expires_at.map(|dt| dt.naive_utc()),
            request.filter_action
        )
        .fetch_one(&mut *tx)
        .await?;

        let filter_id = filter_row.id;

        // Insert filter keywords
        let mut keywords = Vec::new();
        for keyword in request.keywords {
            let keyword_row = sqlx::query!(
                r#"
                INSERT INTO filter_keywords (filter_id, keyword, whole_word)
                VALUES ($1, $2, false)
                RETURNING id, filter_id, keyword, whole_word, created_at
                "#,
                filter_id,
                keyword
            )
            .fetch_one(&mut *tx)
            .await?;

            keywords.push(FilterKeyword {
                id: keyword_row.id,
                filter_id: keyword_row.filter_id,
                keyword: keyword_row.keyword,
                whole_word: keyword_row.whole_word.unwrap_or(false),
                created_at: DateTime::from_naive_utc_and_offset(keyword_row.created_at, Utc),
            });
        }

        // Commit transaction
        tx.commit().await?;

        let filter = Filter {
            id: filter_row.id,
            account_id: filter_row.account_id,
            title: filter_row.title.unwrap_or_default(),
            context: filter_row.context.join(","),
            expires_at: filter_row
                .expires_at
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            filter_action: filter_row.filter_action,
            created_at: DateTime::from_naive_utc_and_offset(filter_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(filter_row.updated_at, Utc),
            keywords,
        };

        info!(
            "Created filter with id: {} for account {} with title: {}",
            filter.id, account_id, filter.title
        );
        Ok(filter)
    }

    /// Gets a filter by ID
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `filter_id` - ID of the filter to retrieve
    ///
    /// # Returns
    ///
    /// Result containing the filter or an error
    pub async fn get_by_id(pool: &PgPool, filter_id: i64) -> Result<Self, FiltersError> {
        trace!("Getting filter by id: {}", filter_id);

        // Get filter
        let filter_row = sqlx::query!(
            r#"
            SELECT id, account_id, title, context, expires_at, filter_action, created_at, updated_at
            FROM filters
            WHERE id = $1
            "#,
            filter_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or(FiltersError::FilterNotFound(filter_id))?;

        // Get filter keywords
        let keyword_rows = sqlx::query!(
            r#"
            SELECT id, filter_id, keyword, whole_word, created_at
            FROM filter_keywords
            WHERE filter_id = $1
            ORDER BY id
            "#,
            filter_id
        )
        .fetch_all(pool)
        .await?;

        let keywords = keyword_rows
            .into_iter()
            .map(|row| FilterKeyword {
                id: row.id,
                filter_id: row.filter_id,
                keyword: row.keyword,
                whole_word: row.whole_word.unwrap_or(false),
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
            })
            .collect();

        let filter = Filter {
            id: filter_row.id,
            account_id: filter_row.account_id,
            title: filter_row.title.unwrap_or_default(),
            context: filter_row.context.join(","),
            expires_at: filter_row
                .expires_at
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            filter_action: filter_row.filter_action,
            created_at: DateTime::from_naive_utc_and_offset(filter_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(filter_row.updated_at, Utc),
            keywords,
        };

        debug!("Retrieved filter with id: {}", filter.id);
        Ok(filter)
    }

    /// Gets all filters for an account
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the account
    ///
    /// # Returns
    ///
    /// Result containing the list of filters or an error
    pub async fn get_by_account(pool: &PgPool, account_id: i64) -> Result<Vec<Self>, FiltersError> {
        trace!("Getting filters for account: {}", account_id);

        // Get filters
        let filter_rows = sqlx::query!(
            r#"
            SELECT id, account_id, title, context, expires_at, filter_action, created_at, updated_at
            FROM filters
            WHERE account_id = $1
            ORDER BY created_at DESC
            "#,
            account_id
        )
        .fetch_all(pool)
        .await?;

        let mut filters = Vec::new();
        for filter_row in filter_rows {
            // Get filter keywords
            let keyword_rows = sqlx::query!(
                r#"
                SELECT id, filter_id, keyword, whole_word, created_at
                FROM filter_keywords
                WHERE filter_id = $1
                ORDER BY id
                "#,
                filter_row.id
            )
            .fetch_all(pool)
            .await?;

            let keywords = keyword_rows
                .into_iter()
                .map(|row| FilterKeyword {
                    id: row.id,
                    filter_id: row.filter_id,
                    keyword: row.keyword,
                    whole_word: row.whole_word.unwrap_or(false),
                    created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                })
                .collect();

            let filter = Filter {
                id: filter_row.id,
                account_id: filter_row.account_id,
                title: filter_row.title.unwrap_or_default(),
                context: filter_row.context.join(","),
                expires_at: filter_row
                    .expires_at
                    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                filter_action: filter_row.filter_action,
                created_at: DateTime::from_naive_utc_and_offset(filter_row.created_at, Utc),
                updated_at: DateTime::from_naive_utc_and_offset(filter_row.updated_at, Utc),
                keywords,
            };

            filters.push(filter);
        }

        debug!(
            "Retrieved {} filters for account {}",
            filters.len(),
            account_id
        );
        Ok(filters)
    }

    /// Updates a filter
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `filter_id` - ID of the filter to update
    /// * `account_id` - ID of the account that owns the filter
    /// * `request` - Update request
    ///
    /// # Returns
    ///
    /// Result containing the updated filter or an error
    pub async fn update(
        pool: &PgPool,
        filter_id: i64,
        account_id: i64,
        request: UpdateFilterRequest,
    ) -> Result<Self, FiltersError> {
        trace!("Updating filter {} for account {}", filter_id, account_id);

        // Validate title if provided
        if let Some(ref title) = request.title {
            if title.trim().is_empty() {
                return Err(FiltersError::Validation(
                    "Filter title cannot be empty".to_string(),
                ));
            }
            if title.len() > 100 {
                return Err(FiltersError::Validation(
                    "Filter title cannot exceed 100 characters".to_string(),
                ));
            }
        }

        // Validate context if provided
        if let Some(ref context) = request.context {
            if context.is_empty() {
                return Err(FiltersError::Validation(
                    "Filter must have at least one context".to_string(),
                ));
            }
        }

        // Start transaction
        let mut tx = pool.begin().await?;

        // Update filter
        let filter_row = sqlx::query!(
            r#"
            UPDATE filters
            SET title = COALESCE($3, title),
                context = COALESCE($4, context),
                expires_at = $5,
                filter_action = COALESCE($6, filter_action),
                updated_at = now()
            WHERE id = $1 AND account_id = $2
            RETURNING id, account_id, title, context, expires_at, filter_action, created_at, updated_at
            "#,
            filter_id,
            account_id,
            request.title,
            request.context.as_deref(),
            request.expires_at.map(|dt| dt.naive_utc()),
            request.filter_action
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(FiltersError::FilterNotFound(filter_id))?;

        // Update keywords if provided
        if let Some(keywords) = request.keywords {
            // Delete existing keywords
            sqlx::query!(
                r#"
                DELETE FROM filter_keywords
                WHERE filter_id = $1
                "#,
                filter_id
            )
            .execute(&mut *tx)
            .await?;

            // Insert new keywords
            for keyword in keywords {
                sqlx::query!(
                    r#"
                    INSERT INTO filter_keywords (filter_id, keyword, whole_word)
                    VALUES ($1, $2, false)
                    "#,
                    filter_id,
                    keyword
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        // Get updated keywords
        let keyword_rows = sqlx::query!(
            r#"
            SELECT id, filter_id, keyword, whole_word, created_at
            FROM filter_keywords
            WHERE filter_id = $1
            ORDER BY id
            "#,
            filter_id
        )
        .fetch_all(&mut *tx)
        .await?;

        let keywords = keyword_rows
            .into_iter()
            .map(|row| FilterKeyword {
                id: row.id,
                filter_id: row.filter_id,
                keyword: row.keyword,
                whole_word: row.whole_word.unwrap_or(false),
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
            })
            .collect();

        // Commit transaction
        tx.commit().await?;

        let filter = Filter {
            id: filter_row.id,
            account_id: filter_row.account_id,
            title: filter_row.title.unwrap_or_default(),
            context: filter_row.context.join(","),
            expires_at: filter_row
                .expires_at
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            filter_action: filter_row.filter_action,
            created_at: DateTime::from_naive_utc_and_offset(filter_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(filter_row.updated_at, Utc),
            keywords,
        };

        info!(
            "Updated filter with id: {} for account {}",
            filter.id, account_id
        );
        Ok(filter)
    }

    /// Deletes a filter
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `filter_id` - ID of the filter to delete
    /// * `account_id` - ID of the account that owns the filter
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn delete(
        pool: &PgPool,
        filter_id: i64,
        account_id: i64,
    ) -> Result<(), FiltersError> {
        trace!("Deleting filter {} for account {}", filter_id, account_id);

        let result = sqlx::query!(
            r#"
            DELETE FROM filters
            WHERE id = $1 AND account_id = $2
            "#,
            filter_id,
            account_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(FiltersError::FilterNotFound(filter_id));
        }

        info!(
            "Deleted filter with id: {} for account {}",
            filter_id, account_id
        );
        Ok(())
    }

    /// Checks if a filter has expired
    ///
    /// # Returns
    ///
    /// True if the filter has expired, false otherwise
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_filter_create_and_delete() {
        // This would require a test database setup
        // For now, just test the struct creation
        let filter = Filter {
            id: 1,
            account_id: 1,
            title: "My Filter".to_string(),
            context: "home".to_string(),
            expires_at: None,
            filter_action: "warn".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            keywords: vec![],
        };

        assert_eq!(filter.account_id, 1);
        assert_eq!(filter.title, "My Filter");
        assert_eq!(filter.context, "home");
        assert!(!filter.is_expired());
    }

    #[tokio::test]
    async fn test_filter_keyword_struct() {
        let keyword = FilterKeyword {
            id: 1,
            filter_id: 1,
            keyword: "spam".to_string(),
            whole_word: false,
            created_at: Utc::now(),
        };

        assert_eq!(keyword.filter_id, 1);
        assert_eq!(keyword.keyword, "spam");
        assert!(!keyword.whole_word);
    }
}
