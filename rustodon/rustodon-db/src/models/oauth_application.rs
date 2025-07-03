//!
//! OAuth Application Model
//!
//! This module provides the OAuth application data model and database operations
//! for the Rustodon server, including creation, retrieval, and validation.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)
//!
//! # Dependencies
//!
//! - `sqlx`: Database operations
//! - `serde`: Serialization
//! - `chrono`: DateTime handling
//!
//! # Usage
//!
//! ```rust
//! use rustodon_db::models::oauth_application::OAuthApplication;
//!
//! let app = OAuthApplication::create(&pool, "My App", "https://example.com/callback", "read write").await?;
//! ```

use crate::error::DbError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tracing::{debug, info};

/// OAuth application data model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OAuthApplication {
    /// Unique identifier
    pub id: i64,
    /// Application name
    pub name: String,
    /// Client ID for OAuth
    pub client_id: String,
    /// Client secret for OAuth
    pub client_secret: String,
    /// Redirect URI for OAuth flow
    pub redirect_uri: Option<String>,
    /// OAuth scopes (optional)
    pub scopes: Option<String>,
    /// Application website (optional)
    pub website: Option<String>,
    /// When the application was created
    pub created_at: Option<NaiveDateTime>,
    /// When the application was last updated
    pub updated_at: Option<NaiveDateTime>,
    /// VAPID key for push notifications (optional)
    pub vapid_key: Option<String>,
}

impl OAuthApplication {
    /// Create a new OAuth application
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `name` - Application name
    /// * `redirect_uri` - Redirect URI for OAuth flow
    /// * `scopes` - OAuth scopes
    /// * `website` - Application website (optional)
    ///
    /// # Returns
    ///
    /// The created OAuth application
    ///
    /// # Examples
    ///
    /// ```rust
    /// let app = OAuthApplication::create(&pool, "My App", "https://example.com/callback", "read write", None).await?;
    /// ```
    pub async fn create(
        pool: &PgPool,
        name: &str,
        redirect_uri: &str,
        scopes: &str,
        website: Option<&str>,
    ) -> Result<Self, DbError> {
        info!("Creating OAuth application: {}", name);

        // Generate client ID and secret
        let client_id = Self::generate_client_id();
        let client_secret = Self::generate_client_secret();

        let result = sqlx::query_as!(
            OAuthApplication,
            r#"
            INSERT INTO oauth_applications (name, client_id, client_secret, redirect_uri, scopes, website)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, client_id, client_secret, redirect_uri, scopes, website, created_at, updated_at, vapid_key
            "#,
            name,
            client_id,
            client_secret,
            redirect_uri,
            scopes,
            website,
        )
        .fetch_one(pool)
        .await?;

        debug!("Created OAuth application with ID: {}", result.id);
        Ok(result)
    }

    /// Get OAuth application by client ID
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `client_id` - Client ID to search for
    ///
    /// # Returns
    ///
    /// The OAuth application if found
    ///
    /// # Examples
    ///
    /// ```rust
    /// let app = OAuthApplication::get_by_client_id(&pool, "client_123").await?;
    /// ```
    pub async fn get_by_client_id(pool: &PgPool, client_id: &str) -> Result<Option<Self>, DbError> {
        debug!("Looking up OAuth application by client ID: {}", client_id);

        let result = sqlx::query_as!(
            OAuthApplication,
            r#"
            SELECT id, name, client_id, client_secret, redirect_uri, scopes, website, created_at, updated_at, vapid_key
            FROM oauth_applications
            WHERE client_id = $1
            "#,
            client_id,
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Get OAuth application by ID
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `id` - Application ID
    ///
    /// # Returns
    ///
    /// The OAuth application if found
    ///
    /// # Examples
    ///
    /// ```rust
    /// let app = OAuthApplication::get_by_id(&pool, 1).await?;
    /// ```
    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, DbError> {
        debug!("Looking up OAuth application by ID: {}", id);

        let result = sqlx::query_as!(
            OAuthApplication,
            r#"
            SELECT id, name, client_id, client_secret, redirect_uri, scopes, website, created_at, updated_at, vapid_key
            FROM oauth_applications
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Get all OAuth applications
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    ///
    /// # Returns
    ///
    /// All OAuth applications
    ///
    /// # Examples
    ///
    /// ```rust
    /// let apps = OAuthApplication::get_all(&pool).await?;
    /// ```
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        debug!("Fetching all OAuth applications");

        let results = sqlx::query_as!(
            OAuthApplication,
            r#"
            SELECT id, name, client_id, client_secret, redirect_uri, scopes, website, created_at, updated_at, vapid_key
            FROM oauth_applications
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    /// Update OAuth application
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `name` - New application name
    /// * `redirect_uri` - New redirect URI
    /// * `scopes` - New OAuth scopes
    /// * `website` - New application website
    ///
    /// # Returns
    ///
    /// The updated OAuth application
    ///
    /// # Examples
    ///
    /// ```rust
    /// let app = app.update(&pool, "New Name", "https://new.example.com/callback", "read write", None).await?;
    /// ```
    pub async fn update(
        &self,
        pool: &PgPool,
        name: &str,
        redirect_uri: &str,
        scopes: &str,
        website: Option<&str>,
    ) -> Result<Self, DbError> {
        info!("Updating OAuth application: {}", self.id);

        let result = sqlx::query_as!(
            OAuthApplication,
            r#"
            UPDATE oauth_applications
            SET name = $1, redirect_uri = $2, scopes = $3, website = $4, updated_at = NOW()
            WHERE id = $5
            RETURNING id, name, client_id, client_secret, redirect_uri, scopes, website, created_at, updated_at, vapid_key
            "#,
            name,
            redirect_uri,
            scopes,
            website,
            self.id,
        )
        .fetch_one(pool)
        .await?;

        debug!("Updated OAuth application: {}", result.id);
        Ok(result)
    }

    /// Delete OAuth application
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    ///
    /// # Returns
    ///
    /// True if the application was deleted, false if not found
    ///
    /// # Examples
    ///
    /// ```rust
    /// let deleted = app.delete(&pool).await?;
    /// ```
    pub async fn delete(&self, pool: &PgPool) -> Result<bool, DbError> {
        info!("Deleting OAuth application: {}", self.id);

        let result = sqlx::query!(
            r#"
            DELETE FROM oauth_applications
            WHERE id = $1
            "#,
            self.id,
        )
        .execute(pool)
        .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            debug!("Deleted OAuth application: {}", self.id);
        } else {
            debug!("OAuth application not found for deletion: {}", self.id);
        }

        Ok(deleted)
    }

    /// Validate client credentials
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `client_id` - Client ID to validate
    /// * `client_secret` - Client secret to validate
    ///
    /// # Returns
    ///
    /// The OAuth application if credentials are valid
    ///
    /// # Examples
    ///
    /// ```rust
    /// let app = OAuthApplication::validate_credentials(&pool, "client_123", "secret_456").await?;
    /// ```
    pub async fn validate_credentials(
        pool: &PgPool,
        client_id: &str,
        client_secret: &str,
    ) -> Result<Option<Self>, DbError> {
        debug!("Validating OAuth credentials for client ID: {}", client_id);

        let result = sqlx::query_as!(
            OAuthApplication,
            r#"
            SELECT id, name, client_id, client_secret, redirect_uri, scopes, website, created_at, updated_at, vapid_key
            FROM oauth_applications
            WHERE client_id = $1 AND client_secret = $2
            "#,
            client_id,
            client_secret,
        )
        .fetch_optional(pool)
        .await?;

        if result.is_some() {
            debug!("OAuth credentials validated for client ID: {}", client_id);
        } else {
            debug!("Invalid OAuth credentials for client ID: {}", client_id);
        }

        Ok(result)
    }

    /// Generate a random client ID
    ///
    /// # Returns
    ///
    /// A random client ID string
    fn generate_client_id() -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let id: String = (0..32).map(|_| rng.sample(Alphanumeric) as char).collect();

        format!("client_{}", id)
    }

    /// Generate a random client secret
    ///
    /// # Returns
    ///
    /// A random client secret string
    fn generate_client_secret() -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let secret: String = (0..64).map(|_| rng.sample(Alphanumeric) as char).collect();

        format!("secret_{}", secret)
    }

    /// Check if the application has a specific scope
    ///
    /// # Arguments
    ///
    /// * `scope` - Scope to check for
    ///
    /// # Returns
    ///
    /// True if the application has the scope
    ///
    /// # Examples
    ///
    /// ```rust
    /// let has_write = app.has_scope("write");
    /// ```
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes
            .as_ref()
            .map_or(false, |s| s.split_whitespace().any(|s| s == scope))
    }

    /// Get all scopes as a vector
    ///
    /// # Returns
    ///
    /// Vector of scopes
    ///
    /// # Examples
    ///
    /// ```rust
    /// let scopes = app.get_scopes();
    /// ```
    pub fn get_scopes(&self) -> Vec<String> {
        self.scopes.as_ref().map_or_else(Vec::new, |s| {
            s.split_whitespace().map(|s| s.to_string()).collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_oauth_application_create() {
        // This would require a test database setup
        // For now, just test the scope methods
        let app = OAuthApplication {
            id: 1,
            name: "Test App".to_string(),
            client_id: "test_client".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: Some("https://example.com/callback".to_string()),
            scopes: Some("read write follow".to_string()),
            website: None,
            created_at: None,
            updated_at: None,
            vapid_key: None,
        };

        assert!(app.has_scope("read"));
        assert!(app.has_scope("write"));
        assert!(app.has_scope("follow"));
        assert!(!app.has_scope("admin"));

        let scopes = app.get_scopes();
        assert_eq!(scopes.len(), 3);
        assert!(scopes.contains(&"read".to_string()));
        assert!(scopes.contains(&"write".to_string()));
        assert!(scopes.contains(&"follow".to_string()));
    }

    #[test]
    fn test_generate_client_id() {
        let client_id = OAuthApplication::generate_client_id();
        assert!(client_id.starts_with("client_"));
        assert_eq!(client_id.len(), 39); // "client_" + 32 chars
    }

    #[test]
    fn test_generate_client_secret() {
        let client_secret = OAuthApplication::generate_client_secret();
        assert!(client_secret.starts_with("secret_"));
        assert_eq!(client_secret.len(), 71); // "secret_" + 64 chars
    }
}
