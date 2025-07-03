//! Database operations and data structures for Rustodon
//!
//! This module provides database operations, data structures, and models
//! for the Rustodon server. It handles all database interactions including
//! CRUD operations, relationships, and data validation.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use thiserror::Error;
use tracing::{info, warn, error, debug, trace};

/// Custom error type for database operations
#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Database connection pool configuration
#[derive(Debug)]
pub struct DbConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: "postgres://localhost/rustodon".to_string(),
            max_connections: 10,
            min_connections: 1,
        }
    }
}

/// Create a new database connection pool
pub async fn create_pool(config: &DbConfig) -> Result<PgPool, DbError> {
    trace!("Creating database pool with config: {:?}", config);

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect(&config.url)
        .await?;

    info!("Database pool created successfully");
    Ok(pool)
}

/// User model representing a Mastodon user account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub display_name: Option<String>,
    pub note: Option<String>,
    pub avatar_url: Option<String>,
    pub header_url: Option<String>,
    pub locked: bool,
    pub bot: bool,
    pub discoverable: bool,
    pub group_account: bool,
    pub last_status_at: Option<NaiveDateTime>,
    pub statuses_count: i64,
    pub followers_count: i64,
    pub following_count: i64,
}

impl User {
    /// Get all users
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        trace!("Fetching all users");
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   locked, bot, discoverable, group_account, last_status_at, statuses_count,
                   followers_count, following_count
            FROM users ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} users", users.len());
        Ok(users)
    }

    /// Get user by ID
    pub async fn get_by_id(pool: &PgPool, user_id: i64) -> Result<Option<Self>, DbError> {
        trace!("Fetching user by ID: {}", user_id);
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   locked, bot, discoverable, group_account, last_status_at, statuses_count,
                   followers_count, following_count
            FROM users WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if user.is_some() {
            debug!("Found user: {}", user_id);
        } else {
            debug!("User not found: {}", user_id);
        }
        Ok(user)
    }

    /// Get user by username
    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Option<Self>, DbError> {
        trace!("Fetching user by username: {}", username);
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   locked, bot, discoverable, group_account, last_status_at, statuses_count,
                   followers_count, following_count
            FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        if user.is_some() {
            debug!("Found user with username: {}", username);
        } else {
            debug!("User not found with username: {}", username);
        }
        Ok(user)
    }

    /// Create a new user
    pub async fn create(
        pool: &PgPool,
        email: &str,
        username: &str,
        password_hash: &str,
        display_name: Option<&str>,
        note: Option<&str>,
    ) -> Result<Self, DbError> {
        trace!("Creating user with username: {}", username);

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, username, password_hash, display_name, note, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            RETURNING id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                      locked, bot, discoverable, group_account, last_status_at, statuses_count,
                      followers_count, following_count
            "#,
            email,
            username,
            password_hash,
            display_name,
            note
        )
        .fetch_one(pool)
        .await?;

        info!("Created user: {} with ID: {}", username, user.id);
        Ok(user)
    }

    /// Update user information
    pub async fn update(
        pool: &PgPool,
        user_id: i64,
        display_name: Option<&str>,
        note: Option<&str>,
        avatar_url: Option<&str>,
        header_url: Option<&str>,
    ) -> Result<Option<Self>, DbError> {
        trace!("Updating user: {}", user_id);

        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET display_name = $2, note = $3, avatar_url = $4, header_url = $5
            WHERE id = $1
            RETURNING id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                      locked, bot, discoverable, group_account, last_status_at, statuses_count,
                      followers_count, following_count
            "#,
            user_id,
            display_name,
            note,
            avatar_url,
            header_url
        )
        .fetch_optional(pool)
        .await?;

        if user.is_some() {
            info!("Updated user: {}", user_id);
        } else {
            warn!("User not found for update: {}", user_id);
        }
        Ok(user)
    }

    /// Delete a user
    pub async fn delete(pool: &PgPool, user_id: i64) -> Result<bool, DbError> {
        trace!("Deleting user: {}", user_id);

        let result = sqlx::query!(
            "DELETE FROM users WHERE id = $1",
            user_id
        )
        .execute(pool)
        .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            info!("Deleted user: {}", user_id);
        } else {
            warn!("User not found for deletion: {}", user_id);
        }
        Ok(deleted)
    }
}

/// ListAccount model for list membership (no id field)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAccount {
    pub list_id: i64,
    pub account_id: i64,
}

impl ListAccount {
    /// Get all list accounts
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        trace!("Fetching all list accounts");
        let list_accounts = sqlx::query_as!(
            ListAccount,
            r#"SELECT list_id, account_id FROM list_accounts"#
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} list accounts", list_accounts.len());
        Ok(list_accounts)
    }

    /// Add accounts to a list
    pub async fn add_accounts(pool: &PgPool, list_id: i64, account_ids: &[i64]) -> Result<(), DbError> {
        trace!("Adding accounts {:?} to list {}", account_ids, list_id);
        for &account_id in account_ids {
            let _ = sqlx::query!(
                "INSERT INTO list_accounts (list_id, account_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                list_id, account_id
            )
            .execute(pool)
            .await?;
        }
        info!("Added {} accounts to list {}", account_ids.len(), list_id);
        Ok(())
    }

    /// Remove accounts from a list
    pub async fn remove_accounts(pool: &PgPool, list_id: i64, account_ids: &[i64]) -> Result<(), DbError> {
        trace!("Removing accounts {:?} from list {}", account_ids, list_id);
        for &account_id in account_ids {
            let _ = sqlx::query!(
                "DELETE FROM list_accounts WHERE list_id = $1 AND account_id = $2",
                list_id, account_id
            )
            .execute(pool)
            .await?;
        }
        info!("Removed {} accounts from list {}", account_ids.len(), list_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    fn test_db_config_default() {
        let config = DbConfig::default();
        assert_eq!(config.url, "postgres://localhost/rustodon");
        assert_eq!(config.max_connections, 10);
    }

    #[tokio::test]
    async fn test_user_operations() {
        // This would require a test database setup
        // For now, just test that the structs can be created
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            created_at: Utc::now().naive_utc(),
            display_name: Some("Test User".to_string()),
            note: None,
            avatar_url: None,
            header_url: None,
            locked: false,
            bot: false,
            discoverable: true,
            group_account: false,
            last_status_at: None,
            statuses_count: 0,
            followers_count: 0,
            following_count: 0,
        };

        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
    }
}
