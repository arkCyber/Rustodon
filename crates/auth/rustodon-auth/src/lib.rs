//!
//! Rustodon Authentication Module
//!
//! This crate provides user authentication, registration, and session management for Rustodon.
//! Integrates with the database module for persistent user storage.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_auth::{register_user, login_user, RegisterRequest, LoginRequest};
//! use rustodon_db::establish_connection;
//! #[tokio::main]
//! async fn main() {
//!     let pool = establish_connection("postgres://localhost/rustodon_test").await.unwrap();
//!     let request = RegisterRequest {
//!         username: "exampleuser123".to_string(),
//!         email: "example123@example.com".to_string(),
//!         password: "password123".to_string(),
//!     };
//!     let result = register_user(&pool, request).await;
//!     if let Ok(session) = result {
//!         println!("User registered with ID: {}", session.user_id);
//!     }
//! }
//! ```
//!
//! # Dependencies
//!
//! - `rustodon_db`: Database operations
//! - `tracing`: Structured logging
//! - `thiserror`: Error handling
//! - `uuid`: Session token generation
//! - `jsonwebtoken`: JWT token handling
//! - `bcrypt`: Password hashing
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use base64::{Engine as _, engine::general_purpose};
use rustodon_db::User;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;
use tracing::{debug, error, info};
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use bcrypt::{hash, verify, DEFAULT_COST};

/// Authentication error type
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User not found: {0}")]
    UserNotFound(String),
    #[error("User already exists: {0}")]
    UserExists(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

/// User registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    /// Username for the new account
    pub username: String,
    /// Email address
    pub email: String,
    /// Plaintext password
    pub password: String,
}

/// User login request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    /// Username or email
    pub username_or_email: String,
    /// Plaintext password
    pub password: String,
}

/// Authentication session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    /// Session token
    pub token: String,
    /// User ID
    pub user_id: i64,
    /// Expiration timestamp
    pub expires_at: chrono::NaiveDateTime,
}

impl AuthSession {
    /// Create a new session
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `expires_in_hours` - Hours until session expires
    ///
    /// # Returns
    /// New AuthSession instance
    pub fn new(user_id: i64, expires_in_hours: u32) -> Self {
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(expires_in_hours as i64);
        let expires_timestamp = expires_at.timestamp();

        // Generate a random component for security
        let random_component = uuid::Uuid::new_v4().to_string();

        // Create token format: base64(user_id:expires_at:random)
        let token_data = format!("{}:{}:{}", user_id, expires_timestamp, random_component);
        let token = general_purpose::STANDARD.encode(token_data.as_bytes());

        Self {
            token,
            user_id,
            expires_at: expires_at.naive_utc(),
        }
    }
}

/// JWT Claims for authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: i64,
    /// Issued at timestamp
    pub iat: i64,
    /// Expiration timestamp
    pub exp: i64,
    /// Username
    pub username: String,
}

/// Authenticated user context
#[derive(Debug, Clone)]
pub struct AuthUser {
    /// User ID
    pub id: i64,
    /// Username
    pub username: String,
    /// Email
    pub email: String,
}

/// JWT configuration
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// Secret key for signing JWT tokens
    pub secret: String,
    /// Token expiration time in hours
    pub expiration_hours: u32,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| {
                "your-secret-key-change-in-production".to_string()
            }),
            expiration_hours: 24,
        }
    }
}

/// Generate JWT token for user
///
/// # Arguments
/// * `user` - User instance
/// * `config` - JWT configuration
///
/// # Returns
/// Result with JWT token string or error
pub fn generate_jwt_token(user: &User, config: &JwtConfig) -> Result<String, AuthError> {
    let now = chrono::Utc::now();
    let exp = now + chrono::Duration::hours(config.expiration_hours as i64);

    let claims = Claims {
        sub: user.id,
        iat: now.timestamp(),
        exp: exp.timestamp(),
        username: user.username.clone(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )
    .map_err(|e| AuthError::Internal(format!("JWT encoding error: {}", e)))
}

/// Verify JWT token and extract claims
///
/// # Arguments
/// * `token` - JWT token string
/// * `config` - JWT configuration
///
/// # Returns
/// Result with Claims or error
pub fn verify_jwt_token(token: &str, config: &JwtConfig) -> Result<Claims, AuthError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_e| AuthError::InvalidCredentials)?;

    Ok(token_data.claims)
}

/// Extract JWT token from Authorization header
///
/// # Arguments
/// * `headers` - HTTP headers
///
/// # Returns
/// Option with token string
pub fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        })
}

/// Authentication middleware for Axum
///
/// This middleware extracts and validates JWT tokens from requests
/// and adds the authenticated user to the request extensions.
///
/// # Arguments
/// * `state` - Application state with database pool
/// * `request` - HTTP request
/// * `next` - Next middleware/handler
///
/// # Returns
/// HTTP response
pub async fn auth_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();

    // Extract token from Authorization header
    let token = match extract_token_from_headers(headers) {
        Some(token) => token,
        None => {
            debug!("No Authorization header found");
            return Ok(next.run(request).await);
        }
    };

    // Verify JWT token
    let claims = match verify_jwt_token(&token, &state.jwt_config) {
        Ok(claims) => claims,
        Err(e) => {
            debug!("Invalid JWT token: {}", e);
            return Ok(next.run(request).await);
        }
    };

    // Get user from database
    let user = match User::get_by_id(&state.pool, claims.sub).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            debug!("User not found for token: {}", claims.sub);
            return Ok(next.run(request).await);
        }
        Err(e) => {
            error!("Database error during authentication: {}", e);
            return Ok(next.run(request).await);
        }
    };

    // Create authenticated user context
    let auth_user = AuthUser {
        id: user.id,
        username: user.username,
        email: user.email,
    };

    // Add authenticated user to request extensions
    let mut request = request;
    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

/// Application state for authentication
#[derive(Debug, Clone)]
pub struct AppState {
    /// Database connection pool
    pub pool: PgPool,
    /// JWT configuration
    pub jwt_config: JwtConfig,
}

impl AppState {
    /// Create new application state
    ///
    /// # Arguments
    /// * `pool` - Database connection pool
    /// * `jwt_config` - JWT configuration (optional, uses default if None)
    ///
    /// # Returns
    /// New AppState instance
    pub fn new(pool: PgPool, jwt_config: Option<JwtConfig>) -> Self {
        Self {
            pool,
            jwt_config: jwt_config.unwrap_or_default(),
        }
    }
}

/// Hashes a password securely using bcrypt
///
/// # Arguments
/// * `password` - The plaintext password
///
/// # Returns
/// A Result with the hashed password or error
///
/// # Examples
/// ```
/// use rustodon_auth::hash_password;
/// let hash = hash_password("password").unwrap();
/// ```
pub fn hash_password(password: &str) -> Result<String, AuthError> {
    info!("Hashing password with bcrypt");
    hash(password, DEFAULT_COST)
        .map_err(|e| AuthError::Internal(format!("Password hashing error: {}", e)))
}

/// Register a new user
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `request` - Registration request
///
/// # Returns
/// Result with AuthSession or AuthError
///
/// # Examples
///
/// ```rust
/// use rustodon_auth::{register_user, RegisterRequest};
/// use rustodon_db::establish_connection;
/// #[tokio::main]
/// async fn main() {
///     let pool = establish_connection("postgres://localhost/rustodon_test").await.unwrap();
///     let request = RegisterRequest {
///         username: "newuser".to_string(),
///         email: "new@example.com".to_string(),
///         password: "password123".to_string(),
///     };
///     let session = register_user(&pool, request).await.unwrap();
///     println!("User registered with ID: {}", session.user_id);
/// }
/// ```
pub async fn register_user(
    pool: &PgPool,
    request: RegisterRequest,
) -> Result<AuthSession, AuthError> {
    info!("Registering new user: {}", request.username);

    // Validate input
    if request.username.len() < 3 {
        return Err(AuthError::Validation(
            "Username must be at least 3 characters".to_string(),
        ));
    }

    if request.password.len() < 8 {
        return Err(AuthError::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Check if user already exists
    if let Some(_) = User::get_by_username(pool, &request.username).await? {
        return Err(AuthError::UserExists(format!(
            "Username {} already exists",
            request.username
        )));
    }

    if let Some(_) = User::get_by_email(pool, &request.email).await? {
        return Err(AuthError::UserExists(format!(
            "Email {} already exists",
            request.email
        )));
    }

    // Hash password
    let password_hash = hash_password(&request.password)?;

    // Save user to database
    let user = User::create(
        pool,
        &request.email,
        &request.username,
        &password_hash,
        None,
        None,
    )
    .await?;

    // Create session
    let session = AuthSession::new(user.id, 24);

    debug!("User registered successfully: {}", request.username);
    Ok(session)
}

/// Authenticate a user
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `request` - Login request
///
/// # Returns
/// Result with AuthSession or AuthError
///
/// # Examples
///
/// ```rust
/// use rustodon_auth::{login_user, LoginRequest};
/// use rustodon_db::establish_connection;
/// #[tokio::main]
/// async fn main() {
///     let pool = establish_connection("postgres://localhost/rustodon_test").await.unwrap();
///     let request = LoginRequest {
///         username_or_email: "exampleuser".to_string(),
///         password: "password123".to_string(),
///     };
///     let result = login_user(&pool, request).await;
///     match result {
///         Ok(session) => println!("User logged in with ID: {}", session.user_id),
///         Err(_) => println!("Login failed"),
///     }
/// }
/// ```
pub async fn login_user(pool: &PgPool, request: LoginRequest) -> Result<AuthSession, AuthError> {
    info!("User login attempt: {}", request.username_or_email);

    // Try to find user by username or email
    let user = if request.username_or_email.contains('@') {
        // Assume it's an email
        User::get_by_email(pool, &request.username_or_email).await?
    } else {
        // Assume it's a username
        User::get_by_username(pool, &request.username_or_email).await?
    };

    let user = user.ok_or_else(|| {
        AuthError::UserNotFound(format!("User {} not found", request.username_or_email))
    })?;

    // Verify password
    if !verify_password(&request.password, &user.password_hash)? {
        return Err(AuthError::InvalidCredentials);
    }

    // Create session
    let session = AuthSession::new(user.id, 24);

    debug!("User logged in successfully: {}", request.username_or_email);
    Ok(session)
}

/// Verify a password against its hash using bcrypt
///
/// # Arguments
/// * `password` - Plaintext password
/// * `hash` - Password hash
///
/// # Returns
/// Result indicating if password matches
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    info!("Verifying password with bcrypt");
    verify(password, hash)
        .map_err(|e| AuthError::Internal(format!("Password verification error: {}", e)))
}

/// Get user by session token
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `token` - Session token
///
/// # Returns
/// Result with optional User or AuthError
pub async fn get_user_by_session(pool: &PgPool, token: &str) -> Result<Option<User>, AuthError> {
    info!("Looking up user by session token");

    // For now, we'll use a simple approach where we store sessions in memory
    // In a real implementation, you'd want to store sessions in the database
    // and check expiration times

    // Parse the token to extract user_id and expiration
    // Token format: base64(user_id:expires_at:random)
    let decoded = match general_purpose::STANDARD.decode(token) {
        Ok(bytes) => String::from_utf8(bytes).unwrap_or_default(),
        Err(_) => return Ok(None),
    };

    let parts: Vec<&str> = decoded.split(':').collect();
    if parts.len() != 3 {
        return Ok(None);
    }

    let user_id: i64 = match parts[0].parse() {
        Ok(id) => id,
        Err(_) => return Ok(None),
    };

    let expires_at: i64 = match parts[1].parse() {
        Ok(expires) => expires,
        Err(_) => return Ok(None),
    };

    // Check if session is expired
    let now = chrono::Utc::now().timestamp();
    if now > expires_at {
        debug!("Session expired for user {}", user_id);
        return Ok(None);
    }

    // Get user from database
    match User::get_by_id(pool, user_id).await {
        Ok(Some(user)) => {
            debug!("Found user {} for session token", user.username);
            Ok(Some(user))
        }
        Ok(None) => {
            debug!("No user found for ID {}", user_id);
            Ok(None)
        }
        Err(e) => {
            error!("Database error looking up user: {}", e);
            Err(AuthError::Database(e))
        }
    }
}

/// Invalidate a session
///
/// # Arguments
/// * `_pool` - Database connection pool
/// * `_token` - Session token to invalidate
///
/// # Returns
/// Result indicating success or failure
pub async fn logout_user(_pool: &PgPool, _token: &str) -> Result<(), AuthError> {
    info!("Logging out user with token");

    // TODO: Implement session invalidation
    // For now, this is a placeholder
    debug!("Session invalidation not implemented yet");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    /// Helper function to establish database connection for tests
    async fn establish_connection(database_url: &str) -> Result<PgPool, sqlx::Error> {
        PgPool::connect(database_url).await
    }

    #[test]
    fn test_hash_password() {
        let hash = hash_password("secret").unwrap();
        // bcrypt hashes start with $2b$ and are much longer
        assert!(hash.starts_with("$2b$"));
        assert!(hash.len() > 20);
    }

    #[test]
    fn test_verify_password() {
        let hash = hash_password("secret").unwrap();
        assert!(verify_password("secret", &hash).unwrap());
        assert!(!verify_password("wrong", &hash).unwrap());
    }

    #[tokio::test]
    async fn test_register_user() {
        let pool = establish_connection("postgres://localhost/rustodon_test")
            .await
            .unwrap();
        let request = RegisterRequest {
            username: format!(
                "testuser_auth_{}",
                uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
            ),
            email: format!(
                "test_auth_{}@example.com",
                uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
            ),
            password: "password123".to_string(),
        };

        let result = register_user(&pool, request).await;
        assert!(result.is_ok());

        let session = result.unwrap();
        assert!(session.user_id > 0);
        assert!(!session.token.is_empty());
    }

    #[tokio::test]
    async fn test_register_user_validation() {
        let pool = establish_connection("postgres://localhost/rustodon_test")
            .await
            .unwrap();
        let request = RegisterRequest {
            username: "ab".to_string(), // Too short
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = register_user(&pool, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_user() {
        let pool = establish_connection("postgres://localhost/rustodon_test")
            .await
            .unwrap();

        // First register a user with unique name
        let uuid_string = uuid::Uuid::new_v4().to_string();
        let unique_id = uuid_string.split('-').next().unwrap();
        let register_request = RegisterRequest {
            username: format!("loginuser_{}", unique_id),
            email: format!("login_{}@example.com", unique_id),
            password: "password123".to_string(),
        };
        register_user(&pool, register_request).await.unwrap();

        // Then try to login
        let login_request = LoginRequest {
            username_or_email: format!("loginuser_{}", unique_id),
            password: "password123".to_string(),
        };

        let result = login_user(&pool, login_request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let pool = establish_connection("postgres://localhost/rustodon_test")
            .await
            .unwrap();
        let request = LoginRequest {
            username_or_email: "nonexistent".to_string(),
            password: "password123".to_string(),
        };

        let result = login_user(&pool, request).await;
        assert!(result.is_err());
    }
}
