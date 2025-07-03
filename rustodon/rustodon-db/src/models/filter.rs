//! Filter model for Rustodon
//!
//! This module defines the Filter model and its database operations.
//! It handles content filtering functionality.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use crate::error::DbError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info, trace};

/// Filter model representing content filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub id: i64,
    pub account_id: i64,
    pub phrase: String,
    pub context: Vec<String>,
    pub expires_at: Option<NaiveDateTime>,
    pub irreversible: bool,
    pub whole_word: bool,
    pub action: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Filter {
    /// Get all filters
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        trace!("Fetching all filters");
        let filters = sqlx::query_as!(
            Filter,
            "SELECT id, account_id, phrase, context, expires_at, irreversible, whole_word, action, created_at, updated_at FROM filters ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} filters", filters.len());
        Ok(filters)
    }

    /// Get filters by account
    pub async fn get_by_account(pool: &PgPool, account_id: i64) -> Result<Vec<Self>, DbError> {
        trace!("Fetching filters for account: {}", account_id);
        let filters = sqlx::query_as!(
            Filter,
            "SELECT id, account_id, phrase, context, expires_at, irreversible, whole_word, action, created_at, updated_at FROM filters WHERE account_id = $1 ORDER BY created_at DESC",
            account_id
        )
        .fetch_all(pool)
        .await?;

        info!(
            "Fetched {} filters for account: {}",
            filters.len(),
            account_id
        );
        Ok(filters)
    }

    /// Get filter by ID
    pub async fn get_by_id(pool: &PgPool, filter_id: i64) -> Result<Option<Self>, DbError> {
        trace!("Fetching filter by ID: {}", filter_id);
        let filter = sqlx::query_as!(
            Filter,
            "SELECT id, account_id, phrase, context, expires_at, irreversible, whole_word, action, created_at, updated_at FROM filters WHERE id = $1",
            filter_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(ref f) = filter {
            info!("Fetched filter: {}", f.id);
        } else {
            debug!("Filter not found: {}", filter_id);
        }
        Ok(filter)
    }

    /// Create a new filter
    pub async fn create(
        pool: &PgPool,
        account_id: i64,
        phrase: &str,
        context: &[String],
        expires_at: Option<NaiveDateTime>,
        irreversible: bool,
        whole_word: bool,
        action: &str,
    ) -> Result<Self, DbError> {
        trace!("Creating filter: {} for account {}", phrase, account_id);
        let filter = sqlx::query_as!(
            Filter,
            "INSERT INTO filters (account_id, phrase, context, expires_at, irreversible, whole_word, action) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, account_id, phrase, context, expires_at, irreversible, whole_word, action, created_at, updated_at",
            account_id,
            phrase,
            context,
            expires_at,
            irreversible,
            whole_word,
            action
        )
        .fetch_one(pool)
        .await?;

        info!("Created filter: {} for account {}", phrase, account_id);
        Ok(filter)
    }

    /// Update a filter
    pub async fn update(
        pool: &PgPool,
        filter_id: i64,
        phrase: &str,
        context: &[String],
        expires_at: Option<NaiveDateTime>,
        irreversible: bool,
        whole_word: bool,
        action: &str,
    ) -> Result<Option<Self>, DbError> {
        trace!("Updating filter: {}", filter_id);
        let filter = sqlx::query_as!(
            Filter,
            "UPDATE filters SET phrase = $2, context = $3, expires_at = $4, irreversible = $5, whole_word = $6, action = $7, updated_at = NOW() WHERE id = $1 RETURNING id, account_id, phrase, context, expires_at, irreversible, whole_word, action, created_at, updated_at",
            filter_id,
            phrase,
            context,
            expires_at,
            irreversible,
            whole_word,
            action
        )
        .fetch_optional(pool)
        .await?;

        if let Some(ref f) = filter {
            info!("Updated filter: {}", f.id);
        } else {
            debug!("Filter not found for update: {}", filter_id);
        }
        Ok(filter)
    }

    /// Delete a filter
    pub async fn delete(pool: &PgPool, filter_id: i64) -> Result<bool, DbError> {
        trace!("Deleting filter: {}", filter_id);
        let result = sqlx::query!("DELETE FROM filters WHERE id = $1", filter_id)
            .execute(pool)
            .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            info!("Deleted filter: {}", filter_id);
        } else {
            debug!("Filter not found for deletion: {}", filter_id);
        }
        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_filter_operations() {
        // This is a basic test structure
        // In a real implementation, you would set up a test database
        // and test the actual CRUD operations
        let filter = Filter {
            id: 1,
            account_id: 1,
            phrase: "spam".to_string(),
            context: vec!["home".to_string(), "notifications".to_string()],
            expires_at: None,
            irreversible: false,
            whole_word: true,
            action: "hide".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(filter.account_id, 1);
        assert_eq!(filter.phrase, "spam");
        assert_eq!(filter.action, "hide");
    }
}
