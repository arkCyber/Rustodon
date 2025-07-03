//!
//! OAuth Access Token Model
//!
//! This module provides the OAuth access token data model and database operations
//! for the Rustodon server, including creation, validation, and revocation.
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
//! use rustodon_db::models::oauth_access_token::OAuthAccessToken;
//!
//! let token = OAuthAccessToken::create(&pool, 1, 1, "read write").await?;
//! ```

use crate::error::DbError;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tracing::{debug, info};

/// OAuth access token data model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OAuthAccessToken {
    /// Unique identifier
    pub id: i64,
    /// OAuth application ID
    pub oauth_application_id: i64,
    /// User ID
    pub resource_owner_id: i64,
    /// Token string
    pub token: String,
    /// Refresh token string (optional)
    pub refresh_token: Option<String>,
    /// Token scopes (optional)
    pub scopes: Option<String>,
    /// When the token expires (optional)
    pub expires_in: Option<i32>,
    /// When the token was created
    pub created_at: NaiveDateTime,
    /// When the token was revoked (optional)
    pub revoked_at: Option<NaiveDateTime>,
}

impl OAuthAccessToken {
    /// Create a new OAuth access token
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `oauth_application_id` - OAuth application ID
    /// * `resource_owner_id` - User ID
    /// * `scopes` - Token scopes
    /// * `expires_in` - Token expiration time in seconds (optional)
    ///
    /// # Returns
    ///
    /// The created OAuth access token
    ///
    /// # Examples
    ///
    /// ```rust
    /// let token = OAuthAccessToken::create(&pool, 1, 1, "read write", Some(7200)).await?;
    /// ```
    pub async fn create(
        pool: &PgPool,
        oauth_application_id: i64,
        resource_owner_id: i64,
        scopes: &str,
        expires_in: Option<i32>,
    ) -> Result<Self, DbError> {
        info!(
            "Creating OAuth access token for user: {}",
            resource_owner_id
        );

        // Generate token and refresh token
        let token = Self::generate_token();
        let refresh_token = Self::generate_token();

        let result = sqlx::query_as!(
            OAuthAccessToken,
            r#"
            INSERT INTO oauth_access_tokens (oauth_application_id, resource_owner_id, token, refresh_token, scopes, expires_in)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, oauth_application_id, resource_owner_id, token, refresh_token, scopes, expires_in, created_at, revoked_at
            "#,
            oauth_application_id,
            resource_owner_id,
            token,
            refresh_token,
            scopes,
            expires_in,
        )
        .fetch_one(pool)
        .await?;

        debug!("Created OAuth access token with ID: {}", result.id);
        Ok(result)
    }

    /// Get OAuth access token by token string
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `token` - Token string to search for
    ///
    /// # Returns
    ///
    /// The OAuth access token if found and not revoked
    ///
    /// # Examples
    ///
    /// ```rust
    /// let token = OAuthAccessToken::get_by_token(&pool, "token_123").await?;
    /// ```
    pub async fn get_by_token(pool: &PgPool, token: &str) -> Result<Option<Self>, DbError> {
        debug!("Looking up OAuth access token by token string");

        let result = sqlx::query_as!(
            OAuthAccessToken,
            r#"
            SELECT id, oauth_application_id, resource_owner_id, token, refresh_token, scopes, expires_in, created_at, revoked_at
            FROM oauth_access_tokens
            WHERE token = $1 AND revoked_at IS NULL
            "#,
            token,
        )
        .fetch_optional(pool)
        .await?;

        if let Some(token_data) = &result {
            // Check if token is expired
            match token_data.expires_in {
                Some(expires_in) => {
                    let expires_at =
                        token_data.created_at + chrono::Duration::seconds(expires_in as i64);
                    if Utc::now().naive_utc() > expires_at {
                        debug!("OAuth access token is expired: {}", token_data.id);
                        return Ok(None);
                    }
                }
                None => {}
            }
        }

        Ok(result)
    }

    /// Get OAuth access token by refresh token
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `refresh_token` - Refresh token string to search for
    ///
    /// # Returns
    ///
    /// The OAuth access token if found and not revoked
    ///
    /// # Examples
    ///
    /// ```rust
    /// let token = OAuthAccessToken::get_by_refresh_token(&pool, "refresh_123").await?;
    /// ```
    pub async fn get_by_refresh_token(
        pool: &PgPool,
        refresh_token: &str,
    ) -> Result<Option<Self>, DbError> {
        debug!("Looking up OAuth access token by refresh token");

        let result = sqlx::query_as!(
            OAuthAccessToken,
            r#"
            SELECT id, oauth_application_id, resource_owner_id, token, refresh_token, scopes, expires_in, created_at, revoked_at
            FROM oauth_access_tokens
            WHERE refresh_token = $1 AND revoked_at IS NULL
            "#,
            refresh_token,
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Get all active tokens for a user
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `user_id` - User ID
    ///
    /// # Returns
    ///
    /// All active OAuth access tokens for the user
    ///
    /// # Examples
    ///
    /// ```rust
    /// let tokens = OAuthAccessToken::get_by_user(&pool, 1).await?;
    /// ```
    pub async fn get_by_user(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, DbError> {
        debug!("Fetching OAuth access tokens for user: {}", user_id);

        let results = sqlx::query_as!(
            OAuthAccessToken,
            r#"
            SELECT id, oauth_application_id, resource_owner_id, token, refresh_token, scopes, expires_in, created_at, revoked_at
            FROM oauth_access_tokens
            WHERE resource_owner_id = $1 AND revoked_at IS NULL
            ORDER BY created_at DESC
            "#,
            user_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    /// Get all active tokens for an OAuth application
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `application_id` - OAuth application ID
    ///
    /// # Returns
    ///
    /// All active OAuth access tokens for the application
    ///
    /// # Examples
    ///
    /// ```rust
    /// let tokens = OAuthAccessToken::get_by_application(&pool, 1).await?;
    /// ```
    pub async fn get_by_application(
        pool: &PgPool,
        application_id: i64,
    ) -> Result<Vec<Self>, DbError> {
        debug!(
            "Fetching OAuth access tokens for application: {}",
            application_id
        );

        let results = sqlx::query_as!(
            OAuthAccessToken,
            r#"
            SELECT id, oauth_application_id, resource_owner_id, token, refresh_token, scopes, expires_in, created_at, revoked_at
            FROM oauth_access_tokens
            WHERE oauth_application_id = $1 AND revoked_at IS NULL
            ORDER BY created_at DESC
            "#,
            application_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    /// Revoke OAuth access token
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    ///
    /// # Returns
    ///
    /// True if the token was revoked, false if not found
    ///
    /// # Examples
    ///
    /// ```rust
    /// let revoked = token.revoke(&pool).await?;
    /// ```
    pub async fn revoke(&self, pool: &PgPool) -> Result<bool, DbError> {
        info!("Revoking OAuth access token: {}", self.id);

        let result = sqlx::query!(
            r#"
            UPDATE oauth_access_tokens
            SET revoked_at = NOW()
            WHERE id = $1 AND revoked_at IS NULL
            "#,
            self.id,
        )
        .execute(pool)
        .await?;

        let revoked = result.rows_affected() > 0;
        if revoked {
            debug!("Revoked OAuth access token: {}", self.id);
        } else {
            debug!("OAuth access token not found for revocation: {}", self.id);
        }

        Ok(revoked)
    }

    /// Revoke all tokens for a user
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `user_id` - User ID
    ///
    /// # Returns
    ///
    /// Number of tokens revoked
    ///
    /// # Examples
    ///
    /// ```rust
    /// let count = OAuthAccessToken::revoke_all_for_user(&pool, 1).await?;
    /// ```
    pub async fn revoke_all_for_user(pool: &PgPool, user_id: i64) -> Result<u64, DbError> {
        info!("Revoking all OAuth access tokens for user: {}", user_id);

        let result = sqlx::query!(
            r#"
            UPDATE oauth_access_tokens
            SET revoked_at = NOW()
            WHERE resource_owner_id = $1 AND revoked_at IS NULL
            "#,
            user_id,
        )
        .execute(pool)
        .await?;

        let count = result.rows_affected();
        debug!(
            "Revoked {} OAuth access tokens for user: {}",
            count, user_id
        );

        Ok(count)
    }

    /// Revoke all tokens for an OAuth application
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `application_id` - OAuth application ID
    ///
    /// # Returns
    ///
    /// Number of tokens revoked
    ///
    /// # Examples
    ///
    /// ```rust
    /// let count = OAuthAccessToken::revoke_all_for_application(&pool, 1).await?;
    /// ```
    pub async fn revoke_all_for_application(
        pool: &PgPool,
        application_id: i64,
    ) -> Result<u64, DbError> {
        info!(
            "Revoking all OAuth access tokens for application: {}",
            application_id
        );

        let result = sqlx::query!(
            r#"
            UPDATE oauth_access_tokens
            SET revoked_at = NOW()
            WHERE oauth_application_id = $1 AND revoked_at IS NULL
            "#,
            application_id,
        )
        .execute(pool)
        .await?;

        let count = result.rows_affected();
        debug!(
            "Revoked {} OAuth access tokens for application: {}",
            count, application_id
        );

        Ok(count)
    }

    /// Check if the token has a specific scope
    ///
    /// # Arguments
    ///
    /// * `scope` - Scope to check for
    ///
    /// # Returns
    ///
    /// True if the token has the scope
    ///
    /// # Examples
    ///
    /// ```rust
    /// let has_write = token.has_scope("write");
    /// ```
    pub fn has_scope(&self, scope: &str) -> bool {
        if let Some(scopes) = &self.scopes {
            scopes.split_whitespace().any(|s| s == scope)
        } else {
            false
        }
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
    /// let scopes = token.get_scopes();
    /// ```
    pub fn get_scopes(&self) -> Vec<String> {
        if let Some(scopes) = &self.scopes {
            scopes.split_whitespace().map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        }
    }

    /// Check if the token is expired
    ///
    /// # Returns
    ///
    /// True if the token is expired
    ///
    /// # Examples
    ///
    /// ```rust
    /// let is_expired = token.is_expired();
    /// ```
    pub fn is_expired(&self) -> bool {
        if let Some(expires_in) = self.expires_in {
            let expires_at = self.created_at + chrono::Duration::seconds(expires_in as i64);
            Utc::now().naive_utc() > expires_at
        } else {
            false // No expiration
        }
    }

    /// Check if the token is revoked
    ///
    /// # Returns
    ///
    /// True if the token is revoked
    ///
    /// # Examples
    ///
    /// ```rust
    /// let is_revoked = token.is_revoked();
    /// ```
    pub fn is_revoked(&self) -> bool {
        self.revoked_at.is_some()
    }

    /// Check if the token is valid (not expired and not revoked)
    ///
    /// # Returns
    ///
    /// True if the token is valid
    ///
    /// # Examples
    ///
    /// ```rust
    /// let is_valid = token.is_valid();
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.is_revoked()
    }

    /// Generate a random token string
    ///
    /// # Returns
    ///
    /// A random token string
    fn generate_token() -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let token: String = (0..64).map(|_| rng.sample(Alphanumeric) as char).collect();

        format!("token_{}", token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_oauth_access_token_scope_methods() {
        let token = OAuthAccessToken {
            id: 1,
            oauth_application_id: 1,
            resource_owner_id: 1,
            token: "test_token".to_string(),
            refresh_token: Some("test_refresh".to_string()),
            scopes: Some("read write follow".to_string()),
            expires_in: Some(7200),
            created_at: Utc::now().naive_utc(),
            revoked_at: None,
        };

        assert!(token.has_scope("read"));
        assert!(token.has_scope("write"));
        assert!(token.has_scope("follow"));
        assert!(!token.has_scope("admin"));

        let scopes = token.get_scopes();
        assert_eq!(scopes.len(), 3);
        assert!(scopes.contains(&"read".to_string()));
        assert!(scopes.contains(&"write".to_string()));
        assert!(scopes.contains(&"follow".to_string()));
    }

    #[test]
    fn test_generate_token() {
        let token = OAuthAccessToken::generate_token();
        assert!(token.starts_with("token_"));
        assert_eq!(token.len(), 70); // "token_" + 64 chars
    }

    #[tokio::test]
    async fn test_token_expiration() {
        let now = Utc::now().naive_utc();

        // Token with 1 hour expiration
        let token = OAuthAccessToken {
            id: 1,
            oauth_application_id: 1,
            resource_owner_id: 1,
            token: "test_token".to_string(),
            refresh_token: None,
            scopes: Some("read".to_string()),
            expires_in: Some(3600), // 1 hour
            created_at: now,
            revoked_at: None,
        };

        // Should not be expired immediately
        assert!(!token.is_expired());
        assert!(token.is_valid());

        // Token with no expiration
        let token_no_expiry = OAuthAccessToken {
            id: 2,
            oauth_application_id: 1,
            resource_owner_id: 1,
            token: "test_token2".to_string(),
            refresh_token: None,
            scopes: Some("read".to_string()),
            expires_in: None,
            created_at: now,
            revoked_at: None,
        };

        assert!(!token_no_expiry.is_expired());
        assert!(token_no_expiry.is_valid());
    }

    #[tokio::test]
    async fn test_token_revocation() {
        let now = Utc::now().naive_utc();

        let token = OAuthAccessToken {
            id: 1,
            oauth_application_id: 1,
            resource_owner_id: 1,
            token: "test_token".to_string(),
            refresh_token: None,
            scopes: Some("read".to_string()),
            expires_in: None,
            created_at: now,
            revoked_at: Some(now), // Revoked
        };

        assert!(token.is_revoked());
        assert!(!token.is_valid());
    }
}
