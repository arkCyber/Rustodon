//!
//! API Endpoints for Rustodon
//!
//! This file defines HTTP endpoints for the Rustodon server, including health checks and instance information.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)
//!
//! # Dependencies
//!
//! - `axum`: Web framework
//! - `serde`: Serialization
//! - `tracing`: Logging
//!
//! # Usage
//!
//! Register these endpoints in the main router.
//!
//! # API Endpoints
//!
//! ## Health & Instance
//! - `GET /health` - Health check
//! - `GET /api/v1/instance` - Instance information
//!
//! ## Authentication
//! - `POST /api/v1/apps` - Register OAuth application
//! - `GET /oauth/authorize` - OAuth authorization
//! - `POST /oauth/token` - OAuth token exchange
//!
//! ## Accounts
//! - `GET /api/v1/accounts/verify_credentials` - Verify account credentials
//! - `GET /api/v1/accounts/:id` - Get account information
//! - `POST /api/v1/accounts` - Create account
//! - `POST /api/v1/accounts/login` - Login
//!
//! ## Statuses
//! - `GET /api/v1/statuses` - Get statuses
//! - `POST /api/v1/statuses` - Create status
//! - `GET /api/v1/statuses/:id` - Get specific status
//! - `DELETE /api/v1/statuses/:id` - Delete status
//!
//! ## Timelines
//! - `GET /api/v1/timelines/home` - Home timeline
//! - `GET /api/v1/timelines/public` - Public timeline
//!
//! ## Follows
//! - `POST /api/v1/accounts/:id/follow` - Follow account
//! - `POST /api/v1/accounts/:id/unfollow` - Unfollow account

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use rustodon_db::models::oauth_access_token::OAuthAccessToken;
use rustodon_db::models::oauth_application::OAuthApplication;
use rustodon_db::get_global_pool;
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use std::collections::HashMap;
use tracing::{debug, info, error};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    limit::RequestBodyLimitLayer,
};
use rustodon_auth::{login_user, LoginRequest};

/// Health check response
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

/// Instance information response
#[derive(Serialize)]
pub struct InstanceResponse {
    pub uri: &'static str,
    pub title: &'static str,
    pub version: &'static str,
    pub description: &'static str,
}

/// App registration request
#[derive(Deserialize)]
pub struct AppRegistrationRequest {
    pub client_name: String,
    pub redirect_uris: String,
    pub scopes: Option<String>,
    pub website: Option<String>,
}

/// App registration response
#[derive(Serialize)]
pub struct AppRegistrationResponse {
    pub id: String,
    pub name: String,
    pub website: Option<String>,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
    pub vapid_key: Option<String>,
}

/// OAuth authorization request
#[derive(Deserialize)]
pub struct OAuthAuthorizeRequest {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
}

/// OAuth token request
#[derive(Deserialize)]
pub struct OAuthTokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub code: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// OAuth token response
#[derive(Serialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub created_at: i64,
}

/// Account information
#[derive(Serialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub bot: bool,
    pub discoverable: Option<bool>,
    pub group: bool,
    pub created_at: String,
    pub note: String,
    pub url: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub followers_count: i32,
    pub following_count: i32,
    pub statuses_count: i32,
    pub last_status_at: Option<String>,
    pub emojis: Vec<String>,
    pub fields: Vec<AccountField>,
}

/// Account field
#[derive(Serialize)]
pub struct AccountField {
    pub name: String,
    pub value: String,
    pub verified_at: Option<String>,
}

/// Status information
#[derive(Serialize)]
pub struct Status {
    pub id: String,
    pub uri: String,
    pub url: String,
    pub account: Account,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub reblog: Option<Box<Status>>,
    pub content: String,
    pub created_at: String,
    pub emojis: Vec<String>,
    pub reblogs_count: i32,
    pub favourites_count: i32,
    pub reblogged: Option<bool>,
    pub favourited: Option<bool>,
    pub muted: Option<bool>,
    pub sensitive: bool,
    pub spoiler_text: String,
    pub visibility: String,
    pub media_attachments: Vec<String>,
    pub mentions: Vec<String>,
    pub tags: Vec<String>,
    pub card: Option<String>,
    pub poll: Option<String>,
    pub application: Option<String>,
    pub language: Option<String>,
    pub pinned: Option<bool>,
    pub bookmarked: Option<bool>,
}

/// Status creation request
#[derive(Deserialize)]
pub struct StatusRequest {
    pub status: String,
    pub in_reply_to_id: Option<String>,
    pub sensitive: Option<bool>,
    pub spoiler_text: Option<String>,
    pub visibility: Option<String>,
    pub language: Option<String>,
}

/// Follow request
#[derive(Deserialize)]
pub struct FollowRequest {
    pub reblogs: Option<bool>,
    pub notify: Option<bool>,
}

/// Follow response
#[derive(Serialize)]
pub struct FollowResponse {
    pub id: String,
    pub following: bool,
    pub showing_reblogs: bool,
    pub notifying: bool,
    pub followed_by: bool,
    pub blocking: bool,
    pub blocked_by: bool,
    pub muting: bool,
    pub muting_notifications: bool,
    pub requested: bool,
    pub domain_blocking: bool,
    pub endorsed: bool,
    pub note: String,
}

/// Login request/response
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i64,
    pub expires_at: String,
}

/// Login endpoint
///
/// POST /api/v1/accounts/login
///
/// # Request
/// {
///   "username_or_email": "string",
///   "password": "string"
/// }
///
/// # Response
/// 200 OK: { "token": "...", "user_id": 1, "expires_at": "..." }
/// 401 Unauthorized: { "error": "Invalid credentials" }
/// 404 Not Found: { "error": "User not found" }
/// 500 Internal Server Error: { "error": "..." }
///
pub async fn login(Json(request): Json<LoginRequest>) -> impl IntoResponse {
    info!("Login API called for: {}", request.username_or_email);
    let pool = match get_global_pool() {
        Some(pool) => pool,
        None => {
            error!("Database pool not initialized");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Database connection failed"})),
            );
        }
    };
    match login_user(pool, request).await {
        Ok(session) => (
            StatusCode::OK,
            axum::Json(serde_json::json!({
                "token": session.token,
                "user_id": session.user_id,
                "expires_at": session.expires_at.to_string(),
            })),
        ),
        Err(e) => {
            error!("Login failed: {}", e);
            let (status, msg) = match e {
                rustodon_auth::AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
                rustodon_auth::AuthError::UserNotFound(_) => (StatusCode::NOT_FOUND, "User not found"),
                rustodon_auth::AuthError::Validation(ref m) => (StatusCode::BAD_REQUEST, m.as_str()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            };
            (status, axum::Json(serde_json::json!({"error": msg})))
        }
    }
}

/// API router with all endpoints
pub fn api_router() -> Router {
    // Create router with middleware layers in correct order
    Router::new()
        // Health and instance endpoints
        .route("/health", get(health))
        .route("/api/v1/instance", get(instance_info))
        // Authentication endpoints
        .route("/api/v1/apps", post(register_app))
        .route("/oauth/authorize", get(oauth_authorize))
        .route("/oauth/token", post(oauth_token))
        // Account endpoints
        .route(
            "/api/v1/accounts/verify_credentials",
            get(verify_credentials),
        )
        .route("/api/v1/accounts/:id", get(get_account))
        .route("/api/v1/accounts", post(create_account))
        .route("/api/v1/accounts/login", post(login))
        // Status endpoints
        .route("/api/v1/statuses", get(get_statuses))
        .route("/api/v1/statuses", post(create_status))
        .route("/api/v1/statuses/:id", get(get_status))
        .route("/api/v1/statuses/:id", delete(delete_status))
        // Timeline endpoints
        .route("/api/v1/timelines/home", get(home_timeline))
        .route("/api/v1/timelines/public", get(public_timeline))
        // Follow endpoints
        .route("/api/v1/accounts/:id/follow", post(follow_account))
        .route("/api/v1/accounts/:id/unfollow", post(unfollow_account))
        // Add middleware layers in correct order
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_credentials(true)
        )
        .layer(CompressionLayer::new())
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)) // 10MB limit
}

/// Health check endpoint
pub async fn health() -> impl IntoResponse {
    debug!("Health check requested");
    Json(HealthResponse { status: "ok" })
}

/// Instance information endpoint
pub async fn instance_info() -> impl IntoResponse {
    debug!("Instance info requested");
    Json(InstanceResponse {
        uri: "localhost",
        title: "Rustodon",
        version: "4.2.0-compatible",
        description: "A Rust implementation of Mastodon backend.",
    })
}

/// Register OAuth application
pub async fn register_app(Json(request): Json<AppRegistrationRequest>) -> impl IntoResponse {
    info!("OAuth app registration requested: {}", request.client_name);

    // Get database pool
    let pool = match get_global_pool() {
        Some(pool) => pool,
        None => {
            error!("Database pool not initialized");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database connection failed"
                })),
            );
        }
    };

    // Create OAuth application in database
    let scopes = request.scopes.unwrap_or_else(|| "read write".to_string());
    let app = match OAuthApplication::create(
        &pool,
        &request.client_name,
        &request.redirect_uris,
        &scopes,
        request.website.as_deref(),
    )
    .await
    {
        Ok(app) => app,
        Err(e) => {
            error!("Failed to create OAuth application: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create OAuth application"
                })),
            );
        }
    };

    let response = AppRegistrationResponse {
        id: app.id.to_string(),
        name: app.name,
        website: app.website,
        redirect_uri: app.redirect_uri.unwrap_or_default(),
        client_id: app.client_id,
        client_secret: app.client_secret,
        vapid_key: app.vapid_key,
    };

    debug!(
        "OAuth application registered successfully with ID: {}",
        app.id
    );
    (StatusCode::OK, axum::Json(to_value(response).unwrap()))
}

/// OAuth authorization endpoint
pub async fn oauth_authorize(Query(params): Query<OAuthAuthorizeRequest>) -> impl IntoResponse {
    info!(
        "OAuth authorization requested for client: {}",
        params.client_id
    );

    // TODO: Implement actual OAuth authorization flow
    // For now, return a mock authorization page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head><title>Authorize Application</title></head>
<body>
<h1>Authorize Application</h1>
<p>Client: {}</p>
<p>Redirect URI: {}</p>
<form method="post" action="/oauth/authorize">
    <input type="hidden" name="client_id" value="{}">
    <input type="hidden" name="redirect_uri" value="{}">
    <input type="hidden" name="scope" value="{}">
    <input type="hidden" name="state" value="{}">
    <button type="submit">Authorize</button>
</form>
</body>
</html>"#,
        params.client_id,
        params.redirect_uri,
        params.client_id,
        params.redirect_uri,
        params.scope.unwrap_or_default(),
        params.state.unwrap_or_default()
    );

    (StatusCode::OK, html)
}

/// OAuth token exchange endpoint
pub async fn oauth_token(Json(request): Json<OAuthTokenRequest>) -> impl IntoResponse {
    info!(
        "OAuth token exchange requested for client: {}",
        request.client_id
    );

    // Get database pool
    let pool = match get_global_pool() {
        Some(pool) => pool,
        None => {
            error!("Database pool not initialized");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database connection failed"
                })),
            );
        }
    };

    // Validate OAuth application credentials
    let app = match OAuthApplication::validate_credentials(
        &pool,
        &request.client_id,
        &request.client_secret,
    )
    .await
    {
        Ok(Some(app)) => app,
        Ok(None) => {
            error!(
                "Invalid OAuth credentials for client: {}",
                request.client_id
            );
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "invalid_client",
                    "error_description": "Invalid client credentials"
                })),
            );
        }
        Err(e) => {
            error!("Database error during OAuth validation: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            );
        }
    };

    // For now, we'll create a token for user ID 1 (mock user)
    // In a real implementation, you would validate the user credentials
    let user_id = 1; // TODO: Implement proper user authentication
    let scopes = app.scopes.clone().unwrap_or_default();

    let token = match OAuthAccessToken::create(
        &pool,
        app.id,
        user_id,
        &scopes,
        Some(7200), // 2 hours expiration
    )
    .await
    {
        Ok(token) => token,
        Err(e) => {
            error!("Failed to create OAuth access token: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create access token"
                })),
            );
        }
    };

    let response = OAuthTokenResponse {
        access_token: token.token,
        token_type: "Bearer".to_string(),
        scope: token.scopes.unwrap_or_default(),
        created_at: token.created_at.and_utc().timestamp(),
    };

    debug!("OAuth token created successfully for user: {}", user_id);
    (StatusCode::OK, axum::Json(to_value(response).unwrap()))
}

/// Verify account credentials
pub async fn verify_credentials() -> impl IntoResponse {
    debug!("Credentials verification requested");

    // TODO: Implement actual credential verification
    let account = Account {
        id: "1".to_string(),
        username: "testuser".to_string(),
        acct: "testuser@localhost".to_string(),
        display_name: "Test User".to_string(),
        locked: false,
        bot: false,
        discoverable: Some(true),
        group: false,
        created_at: "2024-01-01T00:00:00.000Z".to_string(),
        note: "Test account".to_string(),
        url: "https://localhost/@testuser".to_string(),
        avatar: "https://localhost/avatars/original/missing.png".to_string(),
        avatar_static: "https://localhost/avatars/original/missing.png".to_string(),
        header: "https://localhost/headers/original/missing.png".to_string(),
        header_static: "https://localhost/headers/original/missing.png".to_string(),
        followers_count: 0,
        following_count: 0,
        statuses_count: 0,
        last_status_at: None,
        emojis: vec![],
        fields: vec![],
    };

    Json(account)
}

/// Get account information
pub async fn get_account(Path(account_id): Path<String>) -> impl IntoResponse {
    debug!("Account info requested for ID: {}", account_id);

    // TODO: Implement actual account lookup
    let account = Account {
        id: account_id,
        username: "testuser".to_string(),
        acct: "testuser@localhost".to_string(),
        display_name: "Test User".to_string(),
        locked: false,
        bot: false,
        discoverable: Some(true),
        group: false,
        created_at: "2024-01-01T00:00:00.000Z".to_string(),
        note: "Test account".to_string(),
        url: "https://localhost/@testuser".to_string(),
        avatar: "https://localhost/avatars/original/missing.png".to_string(),
        avatar_static: "https://localhost/avatars/original/missing.png".to_string(),
        header: "https://localhost/headers/original/missing.png".to_string(),
        header_static: "https://localhost/headers/original/missing.png".to_string(),
        followers_count: 0,
        following_count: 0,
        statuses_count: 0,
        last_status_at: None,
        emojis: vec![],
        fields: vec![],
    };

    Json(account)
}

/// Create account
pub async fn create_account(Json(request): Json<HashMap<String, String>>) -> impl IntoResponse {
    info!("Account creation requested");

    // TODO: Implement actual account creation
    let account = Account {
        id: "2".to_string(),
        username: request
            .get("username")
            .unwrap_or(&"newuser".to_string())
            .clone(),
        acct: format!(
            "{}@localhost",
            request.get("username").unwrap_or(&"newuser".to_string())
        ),
        display_name: request
            .get("display_name")
            .unwrap_or(&"New User".to_string())
            .clone(),
        locked: false,
        bot: false,
        discoverable: Some(true),
        group: false,
        created_at: chrono::Utc::now().to_rfc3339(),
        note: "".to_string(),
        url: "https://localhost/@newuser".to_string(),
        avatar: "https://localhost/avatars/original/missing.png".to_string(),
        avatar_static: "https://localhost/avatars/original/missing.png".to_string(),
        header: "https://localhost/headers/original/missing.png".to_string(),
        header_static: "https://localhost/headers/original/missing.png".to_string(),
        followers_count: 0,
        following_count: 0,
        statuses_count: 0,
        last_status_at: None,
        emojis: vec![],
        fields: vec![],
    };

    (StatusCode::CREATED, Json(account))
}

/// Get statuses
pub async fn get_statuses(Query(_params): Query<HashMap<String, String>>) -> impl IntoResponse {
    debug!("Statuses requested");

    // TODO: Implement actual status retrieval
    let statuses = vec![Status {
        id: "1".to_string(),
        uri: "https://localhost/users/testuser/statuses/1".to_string(),
        url: "https://localhost/@testuser/1".to_string(),
        account: Account {
            id: "1".to_string(),
            username: "testuser".to_string(),
            acct: "testuser@localhost".to_string(),
            display_name: "Test User".to_string(),
            locked: false,
            bot: false,
            discoverable: Some(true),
            group: false,
            created_at: "2024-01-01T00:00:00.000Z".to_string(),
            note: "Test account".to_string(),
            url: "https://localhost/@testuser".to_string(),
            avatar: "https://localhost/avatars/original/missing.png".to_string(),
            avatar_static: "https://localhost/avatars/original/missing.png".to_string(),
            header: "https://localhost/headers/original/missing.png".to_string(),
            header_static: "https://localhost/headers/original/missing.png".to_string(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 1,
            last_status_at: Some("2024-01-01T00:00:00.000Z".to_string()),
            emojis: vec![],
            fields: vec![],
        },
        in_reply_to_id: None,
        in_reply_to_account_id: None,
        reblog: None,
        content: "<p>Hello, Rustodon!</p>".to_string(),
        created_at: "2024-01-01T00:00:00.000Z".to_string(),
        emojis: vec![],
        reblogs_count: 0,
        favourites_count: 0,
        reblogged: Some(false),
        favourited: Some(false),
        muted: Some(false),
        sensitive: false,
        spoiler_text: "".to_string(),
        visibility: "public".to_string(),
        media_attachments: vec![],
        mentions: vec![],
        tags: vec![],
        card: None,
        poll: None,
        application: None,
        language: Some("en".to_string()),
        pinned: Some(false),
        bookmarked: Some(false),
    }];

    Json(statuses)
}

/// Create status
pub async fn create_status(Json(request): Json<StatusRequest>) -> impl IntoResponse {
    info!("Status creation requested: {}", request.status);

    // TODO: Implement actual status creation
    let status = Status {
        id: "2".to_string(),
        uri: "https://localhost/users/testuser/statuses/2".to_string(),
        url: "https://localhost/@testuser/2".to_string(),
        account: Account {
            id: "1".to_string(),
            username: "testuser".to_string(),
            acct: "testuser@localhost".to_string(),
            display_name: "Test User".to_string(),
            locked: false,
            bot: false,
            discoverable: Some(true),
            group: false,
            created_at: "2024-01-01T00:00:00.000Z".to_string(),
            note: "Test account".to_string(),
            url: "https://localhost/@testuser".to_string(),
            avatar: "https://localhost/avatars/original/missing.png".to_string(),
            avatar_static: "https://localhost/avatars/original/missing.png".to_string(),
            header: "https://localhost/headers/original/missing.png".to_string(),
            header_static: "https://localhost/headers/original/missing.png".to_string(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 2,
            last_status_at: Some(chrono::Utc::now().to_rfc3339()),
            emojis: vec![],
            fields: vec![],
        },
        in_reply_to_id: request.in_reply_to_id,
        in_reply_to_account_id: None,
        reblog: None,
        content: format!("<p>{}</p>", request.status),
        created_at: chrono::Utc::now().to_rfc3339(),
        emojis: vec![],
        reblogs_count: 0,
        favourites_count: 0,
        reblogged: Some(false),
        favourited: Some(false),
        muted: Some(false),
        sensitive: request.sensitive.unwrap_or(false),
        spoiler_text: request.spoiler_text.unwrap_or_default(),
        visibility: request.visibility.unwrap_or_else(|| "public".to_string()),
        media_attachments: vec![],
        mentions: vec![],
        tags: vec![],
        card: None,
        poll: None,
        application: None,
        language: request.language,
        pinned: Some(false),
        bookmarked: Some(false),
    };

    (StatusCode::CREATED, Json(status))
}

/// Get specific status
pub async fn get_status(Path(status_id): Path<String>) -> impl IntoResponse {
    debug!("Status requested for ID: {}", status_id);

    // TODO: Implement actual status lookup
    let status = Status {
        id: status_id,
        uri: "https://localhost/users/testuser/statuses/1".to_string(),
        url: "https://localhost/@testuser/1".to_string(),
        account: Account {
            id: "1".to_string(),
            username: "testuser".to_string(),
            acct: "testuser@localhost".to_string(),
            display_name: "Test User".to_string(),
            locked: false,
            bot: false,
            discoverable: Some(true),
            group: false,
            created_at: "2024-01-01T00:00:00.000Z".to_string(),
            note: "Test account".to_string(),
            url: "https://localhost/@testuser".to_string(),
            avatar: "https://localhost/avatars/original/missing.png".to_string(),
            avatar_static: "https://localhost/avatars/original/missing.png".to_string(),
            header: "https://localhost/headers/original/missing.png".to_string(),
            header_static: "https://localhost/headers/original/missing.png".to_string(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 1,
            last_status_at: Some("2024-01-01T00:00:00.000Z".to_string()),
            emojis: vec![],
            fields: vec![],
        },
        in_reply_to_id: None,
        in_reply_to_account_id: None,
        reblog: None,
        content: "<p>Hello, Rustodon!</p>".to_string(),
        created_at: "2024-01-01T00:00:00.000Z".to_string(),
        emojis: vec![],
        reblogs_count: 0,
        favourites_count: 0,
        reblogged: Some(false),
        favourited: Some(false),
        muted: Some(false),
        sensitive: false,
        spoiler_text: "".to_string(),
        visibility: "public".to_string(),
        media_attachments: vec![],
        mentions: vec![],
        tags: vec![],
        card: None,
        poll: None,
        application: None,
        language: Some("en".to_string()),
        pinned: Some(false),
        bookmarked: Some(false),
    };

    Json(status)
}

/// Delete status
pub async fn delete_status(Path(status_id): Path<String>) -> impl IntoResponse {
    info!("Status deletion requested for ID: {}", status_id);

    // TODO: Implement actual status deletion
    StatusCode::OK
}

/// Get home timeline
pub async fn home_timeline(Query(_params): Query<HashMap<String, String>>) -> impl IntoResponse {
    debug!("Home timeline requested");

    // TODO: Implement actual home timeline
    let statuses = vec![Status {
        id: "1".to_string(),
        uri: "https://localhost/users/testuser/statuses/1".to_string(),
        url: "https://localhost/@testuser/1".to_string(),
        account: Account {
            id: "1".to_string(),
            username: "testuser".to_string(),
            acct: "testuser@localhost".to_string(),
            display_name: "Test User".to_string(),
            locked: false,
            bot: false,
            discoverable: Some(true),
            group: false,
            created_at: "2024-01-01T00:00:00.000Z".to_string(),
            note: "Test account".to_string(),
            url: "https://localhost/@testuser".to_string(),
            avatar: "https://localhost/avatars/original/missing.png".to_string(),
            avatar_static: "https://localhost/avatars/original/missing.png".to_string(),
            header: "https://localhost/headers/original/missing.png".to_string(),
            header_static: "https://localhost/headers/original/missing.png".to_string(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 1,
            last_status_at: Some("2024-01-01T00:00:00.000Z".to_string()),
            emojis: vec![],
            fields: vec![],
        },
        in_reply_to_id: None,
        in_reply_to_account_id: None,
        reblog: None,
        content: "<p>Hello, Rustodon!</p>".to_string(),
        created_at: "2024-01-01T00:00:00.000Z".to_string(),
        emojis: vec![],
        reblogs_count: 0,
        favourites_count: 0,
        reblogged: Some(false),
        favourited: Some(false),
        muted: Some(false),
        sensitive: false,
        spoiler_text: "".to_string(),
        visibility: "public".to_string(),
        media_attachments: vec![],
        mentions: vec![],
        tags: vec![],
        card: None,
        poll: None,
        application: None,
        language: Some("en".to_string()),
        pinned: Some(false),
        bookmarked: Some(false),
    }];

    Json(statuses)
}

/// Get public timeline
pub async fn public_timeline(Query(_params): Query<HashMap<String, String>>) -> impl IntoResponse {
    debug!("Public timeline requested");

    // TODO: Implement actual public timeline
    let statuses = vec![Status {
        id: "1".to_string(),
        uri: "https://localhost/users/testuser/statuses/1".to_string(),
        url: "https://localhost/@testuser/1".to_string(),
        account: Account {
            id: "1".to_string(),
            username: "testuser".to_string(),
            acct: "testuser@localhost".to_string(),
            display_name: "Test User".to_string(),
            locked: false,
            bot: false,
            discoverable: Some(true),
            group: false,
            created_at: "2024-01-01T00:00:00.000Z".to_string(),
            note: "Test account".to_string(),
            url: "https://localhost/@testuser".to_string(),
            avatar: "https://localhost/avatars/original/missing.png".to_string(),
            avatar_static: "https://localhost/avatars/original/missing.png".to_string(),
            header: "https://localhost/headers/original/missing.png".to_string(),
            header_static: "https://localhost/headers/original/missing.png".to_string(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 1,
            last_status_at: Some("2024-01-01T00:00:00.000Z".to_string()),
            emojis: vec![],
            fields: vec![],
        },
        in_reply_to_id: None,
        in_reply_to_account_id: None,
        reblog: None,
        content: "<p>Hello, Rustodon!</p>".to_string(),
        created_at: "2024-01-01T00:00:00.000Z".to_string(),
        emojis: vec![],
        reblogs_count: 0,
        favourites_count: 0,
        reblogged: Some(false),
        favourited: Some(false),
        muted: Some(false),
        sensitive: false,
        spoiler_text: "".to_string(),
        visibility: "public".to_string(),
        media_attachments: vec![],
        mentions: vec![],
        tags: vec![],
        card: None,
        poll: None,
        application: None,
        language: Some("en".to_string()),
        pinned: Some(false),
        bookmarked: Some(false),
    }];

    Json(statuses)
}

/// Follow account
pub async fn follow_account(
    Path(account_id): Path<String>,
    Json(request): Json<FollowRequest>,
) -> impl IntoResponse {
    info!("Follow account requested for ID: {}", account_id);

    // TODO: Implement actual follow functionality
    let response = FollowResponse {
        id: account_id,
        following: true,
        showing_reblogs: request.reblogs.unwrap_or(true),
        notifying: request.notify.unwrap_or(false),
        followed_by: false,
        blocking: false,
        blocked_by: false,
        muting: false,
        muting_notifications: false,
        requested: false,
        domain_blocking: false,
        endorsed: false,
        note: "".to_string(),
    };

    Json(response)
}

/// Unfollow account
pub async fn unfollow_account(Path(account_id): Path<String>) -> impl IntoResponse {
    info!("Unfollow account requested for ID: {}", account_id);

    // TODO: Implement actual unfollow functionality
    let response = FollowResponse {
        id: account_id,
        following: false,
        showing_reblogs: false,
        notifying: false,
        followed_by: false,
        blocking: false,
        blocked_by: false,
        muting: false,
        muting_notifications: false,
        requested: false,
        domain_blocking: false,
        endorsed: false,
        note: "".to_string(),
    };

    Json(response)
}
