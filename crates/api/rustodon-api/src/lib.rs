//! API layer for Rustodon
//!
//! This module provides the HTTP API functionality for the Rustodon server.
//! It handles routing, request processing, and response generation.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use axum::{
    routing::get,
    Router,
    Json,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use std::net::SocketAddr;
use tracing::info;

/// Start the API server
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `addr` - Socket address to bind to
///
/// # Returns
///
/// Result indicating success or failure
pub async fn start_server(pool: PgPool, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Rustodon API server on {}", addr);

    // Create the router
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/api/v1/instance", get(instance_handler))
        .route("/api/v1/accounts", get(accounts_handler))
        .route("/api/v1/statuses", get(statuses_handler))
        .route("/api/v1/timelines/public", get(public_timeline_handler))
        .route("/api/v1/apps", get(apps_handler))
        .with_state(pool);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}

/// Root handler
async fn root_handler() -> &'static str {
    "Rustodon API Server"
}

/// Health check handler
async fn health_handler() -> &'static str {
    "OK"
}

/// Instance information handler
async fn instance_handler() -> impl IntoResponse {
    Json(json!({
        "uri": "rustodon.example.com",
        "title": "Rustodon Instance",
        "short_description": "A high-performance Rust implementation of Mastodon server",
        "description": "Rustodon is a modern, type-safe, and high-performance implementation of the Mastodon server written in Rust.",
        "email": "admin@rustodon.example.com",
        "version": "0.1.0",
        "urls": {
            "streaming_api": "wss://rustodon.example.com"
        },
        "stats": {
            "user_count": 0,
            "status_count": 0,
            "domain_count": 0
        },
        "thumbnail": null,
        "languages": ["en"],
        "registrations": true,
        "approval_required": false,
        "invites_enabled": false,
        "configuration": {
            "statuses": {
                "max_characters": 500,
                "max_media_attachments": 4,
                "characters_reserved_per_url": 23
            },
            "media_attachments": {
                "supported_mime_types": [
                    "image/jpeg",
                    "image/png",
                    "image/gif",
                    "image/webp",
                    "video/mp4",
                    "video/webm"
                ],
                "image_size_limit": 10485760,
                "image_matrix_limit": 16777216,
                "video_size_limit": 41943040,
                "video_frame_rate_limit": 60,
                "video_matrix_limit": 2304000
            },
            "polls": {
                "max_options": 4,
                "max_characters_per_option": 50,
                "min_expiration": 300,
                "max_expiration": 2629746
            }
        },
        "contact_account": null
    }))
}

/// Accounts handler
async fn accounts_handler() -> impl IntoResponse {
    Json(json!([
        {
            "id": "1",
            "username": "admin",
            "acct": "admin@rustodon.example.com",
            "display_name": "Rustodon Admin",
            "locked": false,
            "bot": false,
            "discoverable": true,
            "group": false,
            "created_at": "2025-01-01T00:00:00.000Z",
            "note": "System administrator account",
            "url": "https://rustodon.example.com/@admin",
            "avatar": "https://rustodon.example.com/avatars/original/missing.png",
            "avatar_static": "https://rustodon.example.com/avatars/original/missing.png",
            "header": "https://rustodon.example.com/headers/original/missing.png",
            "header_static": "https://rustodon.example.com/headers/original/missing.png",
            "followers_count": 0,
            "following_count": 0,
            "statuses_count": 0,
            "last_status_at": null,
            "emojis": [],
            "fields": []
        }
    ]))
}

/// Statuses handler
async fn statuses_handler() -> impl IntoResponse {
    Json(json!([
        {
            "id": "1",
            "content": "This is a test status",
            "created_at": "2025-01-01T00:00:00.000Z",
            "account": {
                "id": "1",
                "username": "admin",
                "acct": "admin@rustodon.example.com",
                "display_name": "Rustodon Admin",
                "locked": false,
                "bot": false,
                "discoverable": true,
                "group": false,
                "created_at": "2025-01-01T00:00:00.000Z",
                "note": "System administrator account",
                "url": "https://rustodon.example.com/@admin",
                "avatar": "https://rustodon.example.com/avatars/original/missing.png",
                "avatar_static": "https://rustodon.example.com/avatars/original/missing.png",
                "header": "https://rustodon.example.com/headers/original/missing.png",
                "header_static": "https://rustodon.example.com/headers/original/missing.png",
                "followers_count": 0,
                "following_count": 0,
                "statuses_count": 0,
                "last_status_at": null,
                "emojis": [],
                "fields": []
            }
        }
    ]))
}

/// Public timeline handler
async fn public_timeline_handler() -> impl IntoResponse {
    Json(json!([
        {
            "id": "1",
            "content": "Welcome to the public timeline!",
            "created_at": "2025-01-01T00:00:00.000Z",
            "account": {
                "id": "1",
                "username": "admin",
                "acct": "admin@rustodon.example.com",
                "display_name": "Rustodon Admin",
                "locked": false,
                "bot": false,
                "discoverable": true,
                "group": false,
                "created_at": "2025-01-01T00:00:00.000Z",
                "note": "System administrator account",
                "url": "https://rustodon.example.com/@admin",
                "avatar": "https://rustodon.example.com/avatars/original/missing.png",
                "avatar_static": "https://rustodon.example.com/avatars/original/missing.png",
                "header": "https://rustodon.example.com/headers/original/missing.png",
                "header_static": "https://rustodon.example.com/headers/original/missing.png",
                "followers_count": 0,
                "following_count": 0,
                "statuses_count": 0,
                "last_status_at": null,
                "emojis": [],
                "fields": []
            }
        }
    ]))
}

/// Apps handler
async fn apps_handler() -> impl IntoResponse {
    Json(json!([
        {
            "id": "1",
            "name": "Test App",
            "website": "https://app.example.com",
            "redirect_uri": "https://app.example.com/callback",
            "client_id": "clientid123",
            "client_secret": "secret123",
            "vapid_key": "vapidkey123"
        }
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_root_handler() {
        let response = root_handler().await;
        assert_eq!(response, "Rustodon API Server");
    }

    #[tokio::test]
    async fn test_health_handler() {
        let response = health_handler().await;
        assert_eq!(response, "OK");
    }

    #[tokio::test]
    async fn test_instance_handler() {
        let response = instance_handler().await;
        // The response should be a valid JSON
        assert!(response.into_response().status().is_success());
    }

    #[tokio::test]
    async fn test_accounts_handler() {
        let response = accounts_handler().await;
        // The response should be a valid JSON
        assert!(response.into_response().status().is_success());
    }

    #[tokio::test]
    async fn test_statuses_handler() {
        let response = statuses_handler().await;
        // The response should be a valid JSON
        assert!(response.into_response().status().is_success());
    }

    #[tokio::test]
    async fn test_public_timeline_handler() {
        let response = public_timeline_handler().await;
        // The response should be a valid JSON
        assert!(response.into_response().status().is_success());
    }

    #[tokio::test]
    async fn test_apps_handler() {
        let response = apps_handler().await;
        // The response should be a valid JSON
        assert!(response.into_response().status().is_success());
    }
}
