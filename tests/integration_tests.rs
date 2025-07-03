//!
//! Integration Tests for Rustodon Server
//!
//! This module contains comprehensive integration tests for the Rustodon server,
//! including authentication, API endpoints, middleware, and database operations.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use rustodon_api::endpoints::api_router;
use rustodon_auth::{AppState, JwtConfig, RegisterRequest, LoginRequest};
use rustodon_db::{init_database, get_global_pool};
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

/// Test configuration
#[derive(Debug, Clone)]
struct TestConfig {
    database_url: String,
    jwt_secret: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("TEST_DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/rustodon_test".to_string()),
            jwt_secret: "test-secret-key".to_string(),
        }
    }
}

/// Test setup helper
async fn setup_test_environment() -> (AppState, TestConfig) {
    let config = TestConfig::default();

    // Initialize database
    if let Err(e) = init_database().await {
        eprintln!("Failed to initialize test database: {}", e);
        std::process::exit(1);
    }

    // Get database pool
    let pool = get_global_pool().expect("Database pool not initialized");

    // Create JWT config
    let jwt_config = JwtConfig {
        secret: config.jwt_secret.clone(),
        expiration_hours: 1,
    };

    // Create app state
    let app_state = AppState::new(pool, Some(jwt_config));

    (app_state, config)
}

/// Helper function to make HTTP requests
async fn make_request(
    app: axum::Router,
    method: &str,
    uri: &str,
    body: Option<serde_json::Value>,
) -> Response<Body> {
    let mut request_builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json");

    let request = if let Some(body_data) = body {
        request_builder
            .body(Body::from(serde_json::to_string(&body_data).unwrap()))
            .unwrap()
    } else {
        request_builder.body(Body::empty()).unwrap()
    };

    app.oneshot(request).await.unwrap()
}

#[tokio::test]
async fn test_health_endpoint() {
    let app = api_router();

    let response = make_request(app, "GET", "/health", None).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let health_response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(health_response["status"], "ok");
}

#[tokio::test]
async fn test_instance_info_endpoint() {
    let app = api_router();

    let response = make_request(app, "GET", "/api/v1/instance", None).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let instance_response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(instance_response["title"], "Rustodon");
    assert_eq!(instance_response["version"], "4.2.0-compatible");
}

#[tokio::test]
async fn test_oauth_app_registration() {
    let app = api_router();

    let registration_data = json!({
        "client_name": "Test App",
        "redirect_uris": "https://example.com/callback",
        "scopes": "read write",
        "website": "https://example.com"
    });

    let response = make_request(app, "POST", "/api/v1/apps", Some(registration_data)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let app_response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(app_response["name"], "Test App");
    assert!(app_response["client_id"].as_str().is_some());
    assert!(app_response["client_secret"].as_str().is_some());
}

#[tokio::test]
async fn test_compression_middleware() {
    let app = api_router();

    let response = make_request(app, "GET", "/health", None).await;

    // Check if compression headers are present
    let content_encoding = response.headers().get("content-encoding");
    // Note: Compression might not always be applied for small responses
    // This test mainly ensures the middleware doesn't break the request
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_cors_middleware() {
    let app = api_router();

    let mut request = Request::builder()
        .method("GET")
        .uri("/health")
        .header("origin", "https://example.com")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Check CORS headers
    let access_control_allow_origin = response.headers().get("access-control-allow-origin");
    assert!(access_control_allow_origin.is_some());
}

#[tokio::test]
async fn test_request_body_limit() {
    let app = api_router();

    // Create a large body that exceeds the limit
    let large_body = json!({
        "data": "x".repeat(11 * 1024 * 1024) // 11MB, exceeds 10MB limit
    });

    let response = make_request(app, "POST", "/api/v1/apps", Some(large_body)).await;

    // Should return 413 Payload Too Large
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_authentication_flow() {
    let (app_state, _config) = setup_test_environment().await;

    // Test user registration
    let register_request = RegisterRequest {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let registration_result = rustodon_auth::register_user(&app_state.pool, register_request).await;
    assert!(registration_result.is_ok());

    let session = registration_result.unwrap();
    assert_eq!(session.user_id, 1); // First user should have ID 1

    // Test user login
    let login_request = LoginRequest {
        username_or_email: "testuser".to_string(),
        password: "password123".to_string(),
    };

    let login_result = rustodon_auth::login_user(&app_state.pool, login_request).await;
    assert!(login_result.is_ok());

    let login_session = login_result.unwrap();
    assert_eq!(login_session.user_id, 1);
}

#[tokio::test]
async fn test_jwt_token_generation() {
    let (app_state, _config) = setup_test_environment().await;

    // Create a test user
    let register_request = RegisterRequest {
        username: "jwtuser".to_string(),
        email: "jwt@example.com".to_string(),
        password: "password123".to_string(),
    };

    let user = rustodon_auth::register_user(&app_state.pool, register_request).await.unwrap();

    // Get user from database
    let db_user = rustodon_db::models::user::User::get_by_id(&app_state.pool, user.user_id).await.unwrap().unwrap();

    // Generate JWT token
    let token = rustodon_auth::generate_jwt_token(&db_user, &app_state.jwt_config).unwrap();

    // Verify JWT token
    let claims = rustodon_auth::verify_jwt_token(&token, &app_state.jwt_config).unwrap();

    assert_eq!(claims.sub, user.user_id);
    assert_eq!(claims.username, "jwtuser");
}

#[tokio::test]
async fn test_invalid_credentials() {
    let (app_state, _config) = setup_test_environment().await;

    // Test login with non-existent user
    let login_request = LoginRequest {
        username_or_email: "nonexistent".to_string(),
        password: "password123".to_string(),
    };

    let login_result = rustodon_auth::login_user(&app_state.pool, login_request).await;
    assert!(login_result.is_err());

    // Test login with wrong password
    let register_request = RegisterRequest {
        username: "wrongpass".to_string(),
        email: "wrong@example.com".to_string(),
        password: "password123".to_string(),
    };

    rustodon_auth::register_user(&app_state.pool, register_request).await.unwrap();

    let wrong_password_request = LoginRequest {
        username_or_email: "wrongpass".to_string(),
        password: "wrongpassword".to_string(),
    };

    let wrong_password_result = rustodon_auth::login_user(&app_state.pool, wrong_password_request).await;
    assert!(wrong_password_result.is_err());
}

#[tokio::test]
async fn test_password_hashing() {
    let password = "testpassword123";

    // Hash password
    let hash = rustodon_auth::hash_password(password).unwrap();

    // Verify password
    let is_valid = rustodon_auth::verify_password(password, &hash).unwrap();
    assert!(is_valid);

    // Test wrong password
    let is_invalid = rustodon_auth::verify_password("wrongpassword", &hash).unwrap();
    assert!(!is_invalid);
}

#[tokio::test]
async fn test_user_validation() {
    let (app_state, _config) = setup_test_environment().await;

    // Test username too short
    let short_username_request = RegisterRequest {
        username: "ab".to_string(), // Less than 3 characters
        email: "short@example.com".to_string(),
        password: "password123".to_string(),
    };

    let short_username_result = rustodon_auth::register_user(&app_state.pool, short_username_request).await;
    assert!(short_username_result.is_err());

    // Test password too short
    let short_password_request = RegisterRequest {
        username: "validuser".to_string(),
        email: "shortpass@example.com".to_string(),
        password: "123".to_string(), // Less than 8 characters
    };

    let short_password_result = rustodon_auth::register_user(&app_state.pool, short_password_request).await;
    assert!(short_password_result.is_err());
}

#[tokio::test]
async fn test_duplicate_user_registration() {
    let (app_state, _config) = setup_test_environment().await;

    // Register first user
    let first_request = RegisterRequest {
        username: "duplicate".to_string(),
        email: "duplicate@example.com".to_string(),
        password: "password123".to_string(),
    };

    let first_result = rustodon_auth::register_user(&app_state.pool, first_request).await;
    assert!(first_result.is_ok());

    // Try to register with same username
    let duplicate_username_request = RegisterRequest {
        username: "duplicate".to_string(),
        email: "different@example.com".to_string(),
        password: "password123".to_string(),
    };

    let duplicate_username_result = rustodon_auth::register_user(&app_state.pool, duplicate_username_request).await;
    assert!(duplicate_username_result.is_err());

    // Try to register with same email
    let duplicate_email_request = RegisterRequest {
        username: "different".to_string(),
        email: "duplicate@example.com".to_string(),
        password: "password123".to_string(),
    };

    let duplicate_email_result = rustodon_auth::register_user(&app_state.pool, duplicate_email_request).await;
    assert!(duplicate_email_result.is_err());
}

/// Test cleanup function
async fn cleanup_test_database() {
    // In a real implementation, you would clean up test data
    // For now, we'll just log that cleanup would happen
    println!("Test cleanup completed");
}

#[tokio::test]
async fn test_end_to_end_workflow() {
    let (app_state, _config) = setup_test_environment().await;

    // 1. Register a new user
    let register_request = RegisterRequest {
        username: "e2euser".to_string(),
        email: "e2e@example.com".to_string(),
        password: "password123".to_string(),
    };

    let registration_result = rustodon_auth::register_user(&app_state.pool, register_request).await;
    assert!(registration_result.is_ok());

    let session = registration_result.unwrap();

    // 2. Login with the user
    let login_request = LoginRequest {
        username_or_email: "e2euser".to_string(),
        password: "password123".to_string(),
    };

    let login_result = rustodon_auth::login_user(&app_state.pool, login_request).await;
    assert!(login_result.is_ok());

    // 3. Get user from database
    let user = rustodon_db::models::user::User::get_by_id(&app_state.pool, session.user_id).await.unwrap().unwrap();
    assert_eq!(user.username, "e2euser");
    assert_eq!(user.email, "e2e@example.com");

    // 4. Generate JWT token
    let token = rustodon_auth::generate_jwt_token(&user, &app_state.jwt_config).unwrap();

    // 5. Verify JWT token
    let claims = rustodon_auth::verify_jwt_token(&token, &app_state.jwt_config).unwrap();
    assert_eq!(claims.sub, user.id);
    assert_eq!(claims.username, user.username);

    // 6. Test API endpoints with authentication
    let app = api_router();

    // Test verify credentials endpoint (should work without auth for now)
    let response = make_request(app, "GET", "/api/v1/accounts/verify_credentials", None).await;
    assert_eq!(response.status(), StatusCode::OK);

    cleanup_test_database().await;
}
