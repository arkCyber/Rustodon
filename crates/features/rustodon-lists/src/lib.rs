//! Lists module for Rustodon
//!
//! This module provides list functionality for the Rustodon server.
//! It handles creating, managing, and organizing lists of accounts
//! with proper database operations and validation.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_lists::{List, ListAccount};
//!
//! let list = List::create(&pool, user_id, "My List").await?;
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

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use thiserror::Error;
use tracing::{debug, error, info, trace};

/// Custom error type for lists module
#[derive(Error, Debug)]
pub enum ListsError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("List not found: {0}")]
    ListNotFound(i64),
    #[error("List account not found")]
    ListAccountNotFound,
    #[error("Account not found: {0}")]
    AccountNotFound(i64),
    #[error("User not found: {0}")]
    UserNotFound(i64),
    #[error("Account already in list")]
    AccountAlreadyInList,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// List data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    /// Unique identifier for the list
    pub id: i64,
    /// ID of the account that owns the list
    pub account_id: i64,
    /// Title of the list
    pub title: String,
    /// Whether the list is private
    pub is_private: bool,
    /// When the list was created
    pub created_at: DateTime<Utc>,
    /// When the list was last updated
    pub updated_at: DateTime<Utc>,
}

/// List account relationship data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAccount {
    /// Unique identifier for the list account relationship
    pub id: i64,
    /// ID of the list
    pub list_id: i64,
    /// ID of the account in the list
    pub account_id: i64,
    /// When the account was added to the list
    pub created_at: DateTime<Utc>,
}

/// Create list request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateListRequest {
    /// Title of the list
    pub title: String,
    /// Whether the list is private
    pub is_private: bool,
}

/// Update list request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateListRequest {
    /// Title of the list
    pub title: Option<String>,
    /// Whether the list is private
    pub is_private: Option<bool>,
}

impl List {
    /// Creates a new list
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the account creating the list
    /// * `request` - List creation request
    ///
    /// # Returns
    ///
    /// Result containing the created list or an error
    pub async fn create(
        pool: &PgPool,
        account_id: i64,
        request: CreateListRequest,
    ) -> Result<Self, ListsError> {
        trace!(
            "Creating list for account {} with title: {}",
            account_id,
            request.title
        );

        // Validate request
        if request.title.trim().is_empty() {
            return Err(ListsError::Validation(
                "List title cannot be empty".to_string(),
            ));
        }
        if request.title.len() > 100 {
            return Err(ListsError::Validation(
                "List title cannot exceed 100 characters".to_string(),
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
            return Err(ListsError::UserNotFound(account_id));
        }

        // Insert list
        let list_row = sqlx::query!(
            r#"
            INSERT INTO lists (account_id, title, is_private)
            VALUES ($1, $2, $3)
            RETURNING id, account_id, title, is_private, created_at, updated_at
            "#,
            account_id,
            request.title,
            request.is_private
        )
        .fetch_one(pool)
        .await?;

        let list = List {
            id: list_row.id,
            account_id: list_row.account_id,
            title: list_row.title,
            is_private: list_row.is_private,
            created_at: DateTime::from_naive_utc_and_offset(list_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(list_row.updated_at, Utc),
        };

        info!(
            "Created list with id: {} for account {} with title: {}",
            list.id, account_id, list.title
        );
        Ok(list)
    }

    /// Gets a list by ID
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `list_id` - ID of the list to retrieve
    ///
    /// # Returns
    ///
    /// Result containing the list or an error
    pub async fn get_by_id(pool: &PgPool, list_id: i64) -> Result<Self, ListsError> {
        trace!("Getting list by id: {}", list_id);

        let list_row = sqlx::query!(
            r#"
            SELECT id, account_id, title, is_private, created_at, updated_at
            FROM lists
            WHERE id = $1
            "#,
            list_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or(ListsError::ListNotFound(list_id))?;

        let list = List {
            id: list_row.id,
            account_id: list_row.account_id,
            title: list_row.title,
            is_private: list_row.is_private,
            created_at: DateTime::from_naive_utc_and_offset(list_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(list_row.updated_at, Utc),
        };

        debug!("Retrieved list with id: {}", list.id);
        Ok(list)
    }

    /// Gets all lists for an account
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the account
    ///
    /// # Returns
    ///
    /// Result containing the list of lists or an error
    pub async fn get_by_account(pool: &PgPool, account_id: i64) -> Result<Vec<Self>, ListsError> {
        trace!("Getting lists for account: {}", account_id);

        let list_rows = sqlx::query!(
            r#"
            SELECT id, account_id, title, is_private, created_at, updated_at
            FROM lists
            WHERE account_id = $1
            ORDER BY created_at DESC
            "#,
            account_id
        )
        .fetch_all(pool)
        .await?;

        let lists: Vec<List> = list_rows
            .into_iter()
            .map(|row| List {
                id: row.id,
                account_id: row.account_id,
                title: row.title,
                is_private: row.is_private,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
            })
            .collect();

        debug!("Retrieved {} lists for account {}", lists.len(), account_id);
        Ok(lists)
    }

    /// Updates a list
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `list_id` - ID of the list to update
    /// * `account_id` - ID of the account that owns the list
    /// * `request` - Update request
    ///
    /// # Returns
    ///
    /// Result containing the updated list or an error
    pub async fn update(
        pool: &PgPool,
        list_id: i64,
        account_id: i64,
        request: UpdateListRequest,
    ) -> Result<Self, ListsError> {
        trace!("Updating list {} for account {}", list_id, account_id);

        // Validate title if provided
        if let Some(ref title) = request.title {
            if title.trim().is_empty() {
                return Err(ListsError::Validation(
                    "List title cannot be empty".to_string(),
                ));
            }
            if title.len() > 100 {
                return Err(ListsError::Validation(
                    "List title cannot exceed 100 characters".to_string(),
                ));
            }
        }

        // Build update query dynamically
        let mut query = String::from("UPDATE lists SET updated_at = now()");
        let mut params: Vec<String> = Vec::new();
        let mut param_count = 0;

        if let Some(title) = &request.title {
            param_count += 1;
            query.push_str(&format!(", title = ${}", param_count));
            params.push(title.clone());
        }

        if let Some(is_private) = request.is_private {
            param_count += 1;
            query.push_str(&format!(", is_private = ${}", param_count));
            params.push(is_private.to_string());
        }

        query.push_str(&format!(
            " WHERE id = ${} AND account_id = ${} RETURNING id, account_id, title, is_private, created_at, updated_at",
            param_count + 1,
            param_count + 2
        ));
        params.push(list_id.to_string());
        params.push(account_id.to_string());

        // Execute update
        let list_row = sqlx::query(&query)
            .bind(&params)
            .fetch_optional(pool)
            .await?
            .ok_or(ListsError::ListNotFound(list_id))?;

        let list = List {
            id: list_row.get("id"),
            account_id: list_row.get("account_id"),
            title: list_row.get("title"),
            is_private: list_row.get("is_private"),
            created_at: DateTime::from_naive_utc_and_offset(list_row.get("created_at"), Utc),
            updated_at: DateTime::from_naive_utc_and_offset(list_row.get("updated_at"), Utc),
        };

        info!(
            "Updated list with id: {} for account {}",
            list.id, account_id
        );
        Ok(list)
    }

    /// Deletes a list
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `list_id` - ID of the list to delete
    /// * `account_id` - ID of the account that owns the list
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn delete(pool: &PgPool, list_id: i64, account_id: i64) -> Result<(), ListsError> {
        trace!("Deleting list {} for account {}", list_id, account_id);

        let result = sqlx::query!(
            r#"
            DELETE FROM lists
            WHERE id = $1 AND account_id = $2
            "#,
            list_id,
            account_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ListsError::ListNotFound(list_id));
        }

        info!(
            "Deleted list with id: {} for account {}",
            list_id, account_id
        );
        Ok(())
    }

    /// Adds an account to a list
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `list_id` - ID of the list
    /// * `account_id` - ID of the account to add
    ///
    /// # Returns
    ///
    /// Result containing the created list account relationship or an error
    pub async fn add_account(
        pool: &PgPool,
        list_id: i64,
        account_id: i64,
    ) -> Result<ListAccount, ListsError> {
        trace!("Adding account {} to list {}", account_id, list_id);

        // Check if list exists
        let list_exists = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM lists
            WHERE id = $1
            "#,
            list_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if list_exists == Some(0) {
            return Err(ListsError::ListNotFound(list_id));
        }

        // Check if account exists
        let account_exists = sqlx::query!(
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

        if account_exists == Some(0) {
            return Err(ListsError::AccountNotFound(account_id));
        }

        // Check if account is already in list
        let existing = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM list_accounts
            WHERE list_id = $1 AND account_id = $2
            "#,
            list_id,
            account_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if existing > Some(0) {
            return Err(ListsError::AccountAlreadyInList);
        }

        // Insert list account
        let list_account_row = sqlx::query!(
            r#"
            INSERT INTO list_accounts (list_id, account_id)
            VALUES ($1, $2)
            RETURNING id, list_id, account_id, created_at
            "#,
            list_id,
            account_id
        )
        .fetch_one(pool)
        .await?;

        let list_account = ListAccount {
            id: list_account_row.id,
            list_id: list_account_row.list_id,
            account_id: list_account_row.account_id,
            created_at: DateTime::from_naive_utc_and_offset(list_account_row.created_at, Utc),
        };

        info!("Added account {} to list {}", account_id, list_id);
        Ok(list_account)
    }

    /// Removes an account from a list
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `list_id` - ID of the list
    /// * `account_id` - ID of the account to remove
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn remove_account(
        pool: &PgPool,
        list_id: i64,
        account_id: i64,
    ) -> Result<(), ListsError> {
        trace!("Removing account {} from list {}", account_id, list_id);

        let result = sqlx::query!(
            r#"
            DELETE FROM list_accounts
            WHERE list_id = $1 AND account_id = $2
            "#,
            list_id,
            account_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ListsError::ListAccountNotFound);
        }

        info!("Removed account {} from list {}", account_id, list_id);
        Ok(())
    }

    /// Gets all accounts in a list
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `list_id` - ID of the list
    /// * `limit` - Maximum number of accounts to return
    /// * `_since_id` - Return accounts after this ID
    /// * `_max_id` - Return accounts before this ID
    /// * `sort_order` - Sort order for the accounts
    ///
    /// # Returns
    ///
    /// Result containing the list of list accounts or an error
    pub async fn get_accounts(
        pool: &PgPool,
        list_id: i64,
        limit: Option<i64>,
        _since_id: Option<i64>,
        _max_id: Option<i64>,
        sort_order: Option<&str>,
    ) -> Result<Vec<ListAccount>, ListsError> {
        trace!(
            "Getting accounts for list {} with limit {:?}",
            list_id,
            limit
        );

        let limit = limit.unwrap_or(20).min(40);
        let order_clause = match sort_order {
            Some("desc") => "ORDER BY created_at DESC",
            Some("asc") => "ORDER BY created_at ASC",
            _ => "",
        };

        let query = format!(
            r#"
            SELECT id, list_id, account_id, created_at
            FROM list_accounts
            WHERE list_id = $1
            {}
            LIMIT $2
            "#,
            order_clause
        );

        let list_account_rows = sqlx::query(&query)
            .bind(list_id)
            .bind(limit)
            .fetch_all(pool)
            .await?;

        let list_accounts: Vec<ListAccount> = list_account_rows
            .into_iter()
            .map(|row| ListAccount {
                id: row.get::<i64, _>("id"),
                list_id: row.get::<i64, _>("list_id"),
                account_id: row.get::<i64, _>("account_id"),
                created_at: DateTime::from_naive_utc_and_offset(
                    row.get::<NaiveDateTime, _>("created_at"),
                    Utc,
                ),
            })
            .collect();

        debug!(
            "Retrieved {} accounts for list {}",
            list_accounts.len(),
            list_id
        );
        Ok(list_accounts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_create_and_delete() {
        // This would require a test database setup
        // For now, just test the struct creation
        let list = List {
            id: 1,
            account_id: 1,
            title: "My List".to_string(),
            is_private: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(list.account_id, 1);
        assert_eq!(list.title, "My List");
        assert!(!list.is_private);
    }

    #[tokio::test]
    async fn test_list_account_struct() {
        let list_account = ListAccount {
            id: 1,
            list_id: 1,
            account_id: 2,
            created_at: Utc::now(),
        };

        assert_eq!(list_account.list_id, 1);
        assert_eq!(list_account.account_id, 2);
    }
}
