//!
//! Rustodon OAuth2 Provider Module
//!
//! This crate provides OAuth2 provider functionality for Rustodon, allowing
//! third-party applications to authenticate and access the API.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_oauth::{OAuthProvider, OAuthApp};
//!
//! #[tokio::main]
//! async fn main() {
//!     let provider = OAuthProvider::new();
//!     let app = OAuthApp::create("My App", "https://myapp.com/callback").await.unwrap();
//! }
//! ```
//!
//! # Dependencies
//!
//! - `rustodon_core`: Core types and traits
//! - `rustodon_db`: Database operations
//! - `base64`: Base64 encoding/decoding
//! - `sha2`: SHA-256 hashing
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

/// OAuth2 error types
#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    #[error("Invalid client credentials")]
    InvalidCredentials,
    #[error("Invalid authorization code")]
    InvalidCode,
    #[error("Invalid redirect URI")]
    InvalidRedirectUri,
    #[error("Invalid scope")]
    InvalidScope,
    #[error("Expired authorization code")]
    ExpiredCode,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid token")]
    InvalidToken,
    #[error("Insufficient scope")]
    InsufficientScope,
}

/// OAuth application
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OAuthApp {
    /// Application ID
    pub id: i64,
    /// Application name
    pub name: String,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Redirect URI
    pub redirect_uri: Option<String>,
    /// Scopes
    pub scopes: Option<String>,
    /// Website
    pub website: Option<String>,
    /// Created at timestamp
    pub created_at: Option<chrono::NaiveDateTime>,
    /// Updated at timestamp
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// OAuth authorization code
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OAuthCode {
    /// Authorization code ID
    pub id: i64,
    /// Authorization code
    pub code: String,
    /// Application ID
    pub app_id: i64,
    /// User ID
    pub user_id: i64,
    /// Redirect URI
    pub redirect_uri: Option<String>,
    /// Scopes
    pub scopes: Option<String>,
    /// Expires at timestamp
    pub expires_at: Option<chrono::NaiveDateTime>,
    /// Created at timestamp
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// OAuth access token
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OAuthToken {
    /// Token ID
    pub id: i64,
    /// Access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: Option<String>,
    /// Application ID
    pub app_id: i64,
    /// User ID
    pub user_id: i64,
    /// Scopes
    pub scopes: Option<String>,
    /// Expires at timestamp
    pub expires_at: Option<chrono::NaiveDateTime>,
    /// Created at timestamp
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// OAuth2 authorization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    /// Response type (should be "code")
    pub response_type: String,
    /// Client ID
    pub client_id: String,
    /// Redirect URI
    pub redirect_uri: String,
    /// Scopes
    pub scope: Option<String>,
    /// State parameter for CSRF protection
    pub state: Option<String>,
}

/// OAuth2 token request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequest {
    /// Grant type
    pub grant_type: String,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Authorization code
    pub code: Option<String>,
    /// Redirect URI
    pub redirect_uri: Option<String>,
    /// Refresh token
    pub refresh_token: Option<String>,
}

/// OAuth2 token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    /// Access token
    pub access_token: String,
    /// Token type
    pub token_type: String,
    /// Expires in seconds
    pub expires_in: Option<i64>,
    /// Refresh token
    pub refresh_token: Option<String>,
    /// Scopes
    pub scope: String,
}

/// OAuth2 provider implementation
pub struct OAuthProvider {
    pool: PgPool,
}

impl OAuthProvider {
    /// Create a new OAuth2 provider
    ///
    /// # Arguments
    /// * `pool` - Database connection pool
    ///
    /// # Returns
    /// OAuth2 provider instance
    pub fn new(pool: PgPool) -> Self {
        info!("Initializing OAuth2 provider");
        Self { pool }
    }

    /// Create a new OAuth2 application
    ///
    /// # Arguments
    /// * `name` - Application name
    /// * `redirect_uri` - Redirect URI
    /// * `scopes` - Application scopes
    /// * `website` - Application website (optional)
    ///
    /// # Returns
    /// Result with the created OAuthApp or error
    pub async fn create_app(
        &self,
        name: &str,
        redirect_uri: &str,
        scopes: &str,
        website: Option<&str>,
    ) -> Result<OAuthApp, OAuthError> {
        info!("Creating OAuth2 application: {}", name);

        let client_id = self.generate_client_id();
        let client_secret = self.generate_client_secret();

        let app = sqlx::query_as!(
            OAuthApp,
            r#"
            INSERT INTO oauth_applications (name, client_id, client_secret, redirect_uri, scopes, website, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
            RETURNING id, name, client_id, client_secret, redirect_uri, scopes, website, created_at, updated_at
            "#,
            name,
            client_id,
            client_secret,
            redirect_uri,
            scopes,
            website
        )
        .fetch_one(&self.pool)
        .await?;

        info!("Created OAuth2 application: {} (ID: {})", name, app.id);
        Ok(app)
    }

    /// Get OAuth2 application by client ID
    ///
    /// # Arguments
    /// * `client_id` - Client ID
    ///
    /// # Returns
    /// Result with optional OAuthApp or error
    pub async fn get_app_by_client_id(
        &self,
        client_id: &str,
    ) -> Result<Option<OAuthApp>, OAuthError> {
        let app = sqlx::query_as!(
            OAuthApp,
            r#"
            SELECT id, name, client_id, client_secret, redirect_uri, scopes, website, created_at, updated_at
            FROM oauth_applications
            WHERE client_id = $1
            "#,
            client_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(app)
    }

    /// Validate client credentials
    ///
    /// # Arguments
    /// * `client_id` - Client ID
    /// * `client_secret` - Client secret
    ///
    /// # Returns
    /// Result with boolean indicating if valid or error
    pub async fn validate_credentials(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<bool, OAuthError> {
        let app = self.get_app_by_client_id(client_id).await?;

        match app {
            Some(app) => Ok(app.client_secret == client_secret),
            None => Ok(false),
        }
    }

    /// Create authorization code
    ///
    /// # Arguments
    /// * `app_id` - Application ID
    /// * `user_id` - User ID
    /// * `redirect_uri` - Redirect URI
    /// * `scopes` - Scopes
    ///
    /// # Returns
    /// Result with the created OAuthCode or error
    pub async fn create_authorization_code(
        &self,
        app_id: i64,
        user_id: i64,
        redirect_uri: &str,
        scopes: &str,
    ) -> Result<OAuthCode, OAuthError> {
        info!(
            "Creating authorization code for app {} and user {}",
            app_id, user_id
        );

        let code = self.generate_authorization_code();
        let expires_at = Utc::now() + chrono::Duration::minutes(10);

        let auth_code = sqlx::query_as::<_, OAuthCode>(
            r#"
            INSERT INTO oauth_authorization_codes (code, app_id, user_id, redirect_uri, scopes, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            RETURNING id, code, app_id, user_id, redirect_uri, scopes, expires_at, created_at
            "#
        )
        .bind(&code)
        .bind(app_id)
        .bind(user_id)
        .bind(redirect_uri)
        .bind(scopes)
        .bind(expires_at.naive_utc())
        .fetch_one(&self.pool)
        .await?;

        info!("Created authorization code: {}", code);
        Ok(auth_code)
    }

    /// Exchange authorization code for access token
    ///
    /// # Arguments
    /// * `code` - Authorization code
    /// * `client_id` - Client ID
    /// * `client_secret` - Client secret
    /// * `redirect_uri` - Redirect URI
    ///
    /// # Returns
    /// Result with TokenResponse or error
    pub async fn exchange_code_for_token(
        &self,
        code: &str,
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
    ) -> Result<TokenResponse, OAuthError> {
        info!("Exchanging authorization code for token");

        // Validate client credentials
        if !self.validate_credentials(client_id, client_secret).await? {
            return Err(OAuthError::InvalidCredentials);
        }

        // Get authorization code
        let auth_code = sqlx::query_as::<_, OAuthCode>(
            r#"
            SELECT id, code, app_id, user_id, redirect_uri, scopes, expires_at, created_at
            FROM oauth_authorization_codes
            WHERE code = $1 AND expires_at > NOW()
            "#,
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await?;

        let auth_code = auth_code.ok_or(OAuthError::InvalidCode)?;

        // Validate redirect URI
        if auth_code.redirect_uri.as_deref() != Some(redirect_uri) {
            return Err(OAuthError::InvalidRedirectUri);
        }

        // Get application
        let app = self
            .get_app_by_client_id(client_id)
            .await?
            .ok_or(OAuthError::InvalidCredentials)?;

        // Create access token
        let access_token = self.generate_access_token();
        let refresh_token = self.generate_refresh_token();
        let expires_at = Utc::now() + chrono::Duration::hours(2);

        let token = sqlx::query_as::<_, OAuthToken>(
            r#"
            INSERT INTO oauth_access_tokens (access_token, refresh_token, app_id, user_id, scopes, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            RETURNING id, access_token, refresh_token, app_id, user_id, scopes, expires_at, created_at
            "#
        )
        .bind(access_token)
        .bind(refresh_token)
        .bind(app.id)
        .bind(auth_code.user_id)
        .bind(auth_code.scopes.unwrap_or_default())
        .bind(expires_at.naive_utc())
        .fetch_one(&self.pool)
        .await?;

        // Delete used authorization code
        sqlx::query(
            r#"
            DELETE FROM oauth_authorization_codes
            WHERE id = $1
            "#,
        )
        .bind(auth_code.id)
        .execute(&self.pool)
        .await?;

        info!("Exchanged authorization code for access token");

        Ok(TokenResponse {
            access_token: token.access_token,
            token_type: "Bearer".to_string(),
            expires_in: Some(7200), // 2 hours
            refresh_token: token.refresh_token,
            scope: token.scopes.unwrap_or_default(),
        })
    }

    /// Validate access token
    ///
    /// # Arguments
    /// * `access_token` - Access token
    ///
    /// # Returns
    /// Result with optional OAuthToken or error
    pub async fn validate_access_token(
        &self,
        access_token: &str,
    ) -> Result<Option<OAuthToken>, OAuthError> {
        let token = sqlx::query_as::<_, OAuthToken>(
            r#"
            SELECT id, access_token, refresh_token, app_id, user_id, scopes, expires_at, created_at
            FROM oauth_access_tokens
            WHERE access_token = $1 AND (expires_at IS NULL OR expires_at > NOW())
            "#,
        )
        .bind(access_token)
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    /// Check if token has required scope
    ///
    /// # Arguments
    /// * `token` - OAuth token
    /// * `required_scope` - Required scope
    ///
    /// # Returns
    /// Result with boolean indicating if scope is sufficient or error
    pub fn has_scope(&self, token: &OAuthToken, required_scope: &str) -> Result<bool, OAuthError> {
        let token_scopes: Vec<&str> = token
            .scopes
            .as_ref()
            .map(|s| s.split(' ').collect::<Vec<&str>>())
            .unwrap_or_default();
        let required_scopes: Vec<&str> = required_scope.split(' ').collect();

        for required_scope in required_scopes {
            if !token_scopes.contains(&required_scope) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Generate client ID
    fn generate_client_id(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(Uuid::new_v4().to_string().as_bytes());
        hasher.update(chrono::Utc::now().timestamp().to_string().as_bytes());
        hex::encode(hasher.finalize())[..32].to_string()
    }

    /// Generate client secret
    fn generate_client_secret(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(Uuid::new_v4().to_string().as_bytes());
        hasher.update(
            chrono::Utc::now()
                .timestamp_nanos_opt()
                .unwrap_or(0)
                .to_string()
                .as_bytes(),
        );
        hex::encode(hasher.finalize())[..64].to_string()
    }

    /// Generate authorization code
    fn generate_authorization_code(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(Uuid::new_v4().to_string().as_bytes());
        hasher.update(chrono::Utc::now().timestamp_millis().to_string().as_bytes());
        hex::encode(hasher.finalize())[..32].to_string()
    }

    /// Generate access token
    fn generate_access_token(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(Uuid::new_v4().to_string().as_bytes());
        hasher.update(chrono::Utc::now().timestamp_micros().to_string().as_bytes());
        hex::encode(hasher.finalize())[..64].to_string()
    }

    /// Generate refresh token
    fn generate_refresh_token(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(Uuid::new_v4().to_string().as_bytes());
        hasher.update(
            chrono::Utc::now()
                .timestamp_nanos_opt()
                .unwrap_or(0)
                .to_string()
                .as_bytes(),
        );
        hex::encode(hasher.finalize())[..64].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_oauth_provider_creation() {
        // This would require a test database setup
        // For now, just test the token generation functions
        let provider = OAuthProvider::new(
            sqlx::PgPool::connect("postgres://localhost/test")
                .await
                .unwrap(),
        );

        let client_id = provider.generate_client_id();
        let client_secret = provider.generate_client_secret();

        assert_eq!(client_id.len(), 32);
        assert_eq!(client_secret.len(), 64);
    }
}
