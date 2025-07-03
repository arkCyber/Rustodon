//! API layer for Rustodon
//!
//! This module provides the HTTP API functionality for the Rustodon server.
//! It handles routing, request processing, and response generation.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rustodon_auth::{login_user, register_user, LoginRequest, RegisterRequest};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use std::net::SocketAddr;
use tracing::{debug, error, info};

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

/// Status creation request
#[derive(Debug, Deserialize)]
pub struct StatusRequest {
    pub status: String,
    pub visibility: Option<String>,
    pub in_reply_to_id: Option<String>,
    pub media_ids: Option<Vec<String>>,
    pub sensitive: Option<bool>,
    pub spoiler_text: Option<String>,
    pub language: Option<String>,
}

/// Follow request
#[derive(Debug, Deserialize)]
pub struct FollowRequest {
    pub reblogs: Option<bool>,
    pub notify: Option<bool>,
}

/// Favorite request
#[derive(Debug, Deserialize)]
pub struct FavoriteRequest {
    pub status_id: String,
}

/// Reblog request
#[derive(Debug, Deserialize)]
pub struct ReblogRequest {
    pub status_id: String,
    pub visibility: Option<String>,
}

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
pub async fn start_server(
    pool: PgPool,
    addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Rustodon API server on {}", addr);

    let state = AppState { pool };

    // Create the router with POST support
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/api/v1/instance", get(instance_handler))
        .route(
            "/api/v1/accounts",
            get(accounts_handler).post(register_handler),
        )
        .route(
            "/api/v1/statuses",
            get(statuses_handler).post(create_status_handler),
        )
        .route("/api/v1/statuses/:id", get(get_status_handler))
        .route(
            "/api/v1/statuses/:id/favourite",
            post(favorite_status_handler),
        )
        .route(
            "/api/v1/statuses/:id/unfavourite",
            post(unfavorite_status_handler),
        )
        .route("/api/v1/statuses/:id/reblog", post(reblog_status_handler))
        .route(
            "/api/v1/statuses/:id/unreblog",
            post(unreblog_status_handler),
        )
        .route(
            "/api/v1/statuses/:id/bookmark",
            post(bookmark_status_handler),
        )
        .route(
            "/api/v1/statuses/:id/unbookmark",
            post(unbookmark_status_handler),
        )
        .route("/api/v1/timelines/public", get(public_timeline_handler))
        .route("/api/v1/apps", get(apps_handler))
        // Authentication endpoints
        .route("/api/v1/auth/register", post(register_handler))
        .route("/api/v1/auth/login", post(login_handler))
        // User relationship endpoints
        .route("/api/v1/accounts/:id/follow", post(follow_account_handler))
        .route(
            "/api/v1/accounts/:id/unfollow",
            post(unfollow_account_handler),
        )
        .route("/api/v1/accounts/:id/block", post(block_account_handler))
        .route(
            "/api/v1/accounts/:id/unblock",
            post(unblock_account_handler),
        )
        .route("/api/v1/accounts/:id/mute", post(mute_account_handler))
        .route("/api/v1/accounts/:id/unmute", post(unmute_account_handler))
        .route("/api/v1/accounts/:id/followers", get(followers_handler))
        .route("/api/v1/accounts/:id/following", get(following_handler))
        // Search endpoint
        .route("/api/v1/search", get(search_handler))
        .route("/api/v1/accounts/search", get(accounts_search_handler))
        // Notifications endpoint
        .route("/api/v1/notifications", get(notifications_handler))
        // Media upload endpoint
        .route("/api/v1/media", post(upload_media_handler))
        // Lists endpoints
        .route(
            "/api/v1/lists",
            get(lists_handler).post(create_list_handler),
        )
        // Conversations endpoints
        .route("/api/v1/conversations", get(conversations_handler))
        // Bookmarks endpoints
        .route("/api/v1/bookmarks", get(bookmarks_handler))
        // Polls endpoints
        .route("/api/v1/polls/:id/votes", post(vote_poll_handler))
        // Trends endpoints
        .route("/api/v1/trends/tags", get(trending_tags_handler))
        .route("/api/v1/trends/statuses", get(trending_statuses_handler))
        .with_state(state);

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

/// User registration handler
async fn register_handler(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> impl IntoResponse {
    debug!(
        "Handling user registration request for: {}",
        request.username
    );

    match register_user(&state.pool, request).await {
        Ok(session) => {
            info!("User registered successfully");
            (
                StatusCode::CREATED,
                Json(json!({
                    "success": true,
                    "data": {
                        "user_id": session.user_id,
                        "token": session.token,
                        "expires_at": session.expires_at
                    },
                    "error": null
                })),
            )
        }
        Err(e) => {
            error!("Registration failed: {:?}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "data": null,
                    "error": e.to_string()
                })),
            )
        }
    }
}

/// User login handler
async fn login_handler(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    debug!(
        "Handling user login request for: {}",
        request.username_or_email
    );

    match login_user(&state.pool, request).await {
        Ok(session) => {
            info!("User logged in successfully");
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "data": {
                        "user_id": session.user_id,
                        "token": session.token,
                        "expires_at": session.expires_at
                    },
                    "error": null
                })),
            )
        }
        Err(e) => {
            error!("Login failed: {:?}", e);
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "data": null,
                    "error": e.to_string()
                })),
            )
        }
    }
}

/// Create status handler
async fn create_status_handler(
    State(state): State<AppState>,
    Json(request): Json<StatusRequest>,
) -> impl IntoResponse {
    debug!("Handling status creation request");

    // For now, return a mock response
    // TODO: Implement actual status creation
    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "data": {
                "id": "1",
                "content": request.status,
                "created_at": chrono::Utc::now().to_rfc3339(),
                "visibility": request.visibility.unwrap_or_else(|| "public".to_string()),
                "sensitive": request.sensitive.unwrap_or(false),
                "spoiler_text": request.spoiler_text,
                "language": request.language.clone().unwrap_or_else(|| "en".to_string()),
                "account": {
                    "id": "1",
                    "username": "testuser",
                    "acct": "testuser@rustodon.example.com",
                    "display_name": "Test User",
                    "locked": false,
                    "bot": false,
                    "discoverable": true,
                    "group": false,
                    "created_at": "2025-01-01T00:00:00.000Z",
                    "note": "Test user account",
                    "url": "https://rustodon.example.com/@testuser",
                    "avatar": "https://rustodon.example.com/avatars/original/missing.png",
                    "avatar_static": "https://rustodon.example.com/avatars/original/missing.png",
                    "header": "https://rustodon.example.com/headers/original/missing.png",
                    "header_static": "https://rustodon.example.com/headers/original/missing.png",
                    "followers_count": 0,
                    "following_count": 0,
                    "statuses_count": 1,
                    "last_status_at": chrono::Utc::now().to_rfc3339(),
                    "emojis": [],
                    "fields": []
                },
                "media_attachments": [],
                "mentions": [],
                "tags": [],
                "emojis": [],
                "reblogs_count": 0,
                "favourites_count": 0,
                "replies_count": 0,
                "url": "https://rustodon.example.com/@testuser/1",
                "in_reply_to_id": request.in_reply_to_id,
                "in_reply_to_account_id": null,
                "reblog": null,
                "poll": null,
                "card": null,
                "application": null,
                "language": request.language.clone().unwrap_or_else(|| "en".to_string()),
                "muted": false,
                "reblogged": false,
                "favourited": false,
                "bookmarked": false,
                "pinned": false
            },
            "error": null
        })),
    )
}

/// Favorite status handler
async fn favorite_status_handler(
    State(_state): State<AppState>,
    Path(status_id): Path<String>,
) -> impl IntoResponse {
    debug!("Handling favorite status request for status: {}", status_id);

    // TODO: Implement actual favorite functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": status_id,
                "favourited": true,
                "favourites_count": 1
            },
            "error": null
        })),
    )
}

/// Unfavorite status handler
async fn unfavorite_status_handler(
    State(_state): State<AppState>,
    Path(status_id): Path<String>,
) -> impl IntoResponse {
    debug!(
        "Handling unfavorite status request for status: {}",
        status_id
    );

    // TODO: Implement actual unfavorite functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": status_id,
                "favourited": false,
                "favourites_count": 0
            },
            "error": null
        })),
    )
}

/// Reblog status handler
async fn reblog_status_handler(
    State(_state): State<AppState>,
    Path(status_id): Path<String>,
) -> impl IntoResponse {
    debug!("Handling reblog status request for status: {}", status_id);

    // TODO: Implement actual reblog functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": "2",
                "reblogged": true,
                "reblogs_count": 1
            },
            "error": null
        })),
    )
}

/// Unreblog status handler
async fn unreblog_status_handler(
    State(_state): State<AppState>,
    Path(status_id): Path<String>,
) -> impl IntoResponse {
    debug!("Handling unreblog status request for status: {}", status_id);

    // TODO: Implement actual unreblog functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": status_id,
                "reblogged": false,
                "reblogs_count": 0
            },
            "error": null
        })),
    )
}

/// Follow account handler
async fn follow_account_handler(
    State(_state): State<AppState>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    debug!(
        "Handling follow account request for account: {}",
        account_id
    );

    // TODO: Implement actual follow functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": account_id,
                "following": true,
                "followed_by": false,
                "requested": false,
                "delivery_following": true,
                "notifying": false,
                "showing_reblogs": true,
                "blocking": false,
                "blocked_by": false,
                "muting": false,
                "muting_notifications": false,
                "requested": false,
                "domain_blocking": false,
                "endorsed": false,
                "note": ""
            },
            "error": null
        })),
    )
}

/// Unfollow account handler
async fn unfollow_account_handler(
    State(_state): State<AppState>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    debug!(
        "Handling unfollow account request for account: {}",
        account_id
    );

    // TODO: Implement actual unfollow functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": account_id,
                "following": false,
                "followed_by": false,
                "requested": false,
                "delivery_following": false,
                "notifying": false,
                "showing_reblogs": false,
                "blocking": false,
                "blocked_by": false,
                "muting": false,
                "muting_notifications": false,
                "requested": false,
                "domain_blocking": false,
                "endorsed": false,
                "note": ""
            },
            "error": null
        })),
    )
}

/// Block account handler
async fn block_account_handler(
    State(_state): State<AppState>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    debug!("Handling block account request for account: {}", account_id);

    // TODO: Implement actual block functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": account_id,
                "blocking": true
            },
            "error": null
        })),
    )
}

/// Unblock account handler
async fn unblock_account_handler(
    State(_state): State<AppState>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    debug!(
        "Handling unblock account request for account: {}",
        account_id
    );

    // TODO: Implement actual unblock functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": account_id,
                "blocking": false
            },
            "error": null
        })),
    )
}

/// Mute account handler
async fn mute_account_handler(
    State(_state): State<AppState>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    debug!("Handling mute account request for account: {}", account_id);

    // TODO: Implement actual mute functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": account_id,
                "muting": true,
                "muting_notifications": false
            },
            "error": null
        })),
    )
}

/// Unmute account handler
async fn unmute_account_handler(
    State(_state): State<AppState>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    debug!(
        "Handling unmute account request for account: {}",
        account_id
    );

    // TODO: Implement actual unmute functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": account_id,
                "muting": false,
                "muting_notifications": false
            },
            "error": null
        })),
    )
}

/// Search handler
async fn search_handler() -> impl IntoResponse {
    debug!("Handling search request");

    // TODO: Implement actual search functionality
    Json(json!({
        "success": true,
        "data": {
            "accounts": [],
            "statuses": [],
            "hashtags": []
        },
        "error": null
    }))
}

/// Notifications handler
async fn notifications_handler() -> impl IntoResponse {
    debug!("Handling notifications request");

    // TODO: Implement actual notifications functionality
    Json(json!({
        "success": true,
        "data": [],
        "error": null
    }))
}

/// Upload media handler
async fn upload_media_handler() -> impl IntoResponse {
    debug!("Handling media upload request");

    // TODO: Implement actual media upload functionality
    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "data": {
                "id": "1",
                "type": "image",
                "url": "https://rustodon.example.com/media/1.jpg",
                "preview_url": "https://rustodon.example.com/media/1_preview.jpg",
                "remote_url": null,
                "preview_remote_url": null,
                "text_url": null,
                "meta": {
                    "original": {
                        "width": 800,
                        "height": 600,
                        "size": "800x600",
                        "aspect": 1.3333333333333333
                    },
                    "small": {
                        "width": 400,
                        "height": 300,
                        "size": "400x300",
                        "aspect": 1.3333333333333333
                    }
                },
                "description": null,
                "blurhash": "U9S%*#~q"
            },
            "error": null
        })),
    )
}

/// Lists handler
async fn lists_handler() -> impl IntoResponse {
    debug!("Handling lists request");

    // TODO: Implement actual lists functionality
    Json(json!({
        "success": true,
        "data": [],
        "error": null
    }))
}

/// Create list handler
async fn create_list_handler() -> impl IntoResponse {
    debug!("Handling create list request");

    // TODO: Implement actual list creation functionality
    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "data": {
                "id": "1",
                "title": "New List",
                "replies_policy": "followed"
            },
            "error": null
        })),
    )
}

/// Conversations handler
async fn conversations_handler() -> impl IntoResponse {
    debug!("Handling conversations request");

    // TODO: Implement actual conversations functionality
    Json(json!({
        "success": true,
        "data": [],
        "error": null
    }))
}

/// Bookmarks handler
async fn bookmarks_handler() -> impl IntoResponse {
    debug!("Handling bookmarks request");

    // TODO: Implement actual bookmarks functionality
    Json(json!({
        "success": true,
        "data": [],
        "error": null
    }))
}

/// Bookmark status handler
async fn bookmark_status_handler(
    State(_state): State<AppState>,
    Path(status_id): Path<String>,
) -> impl IntoResponse {
    debug!("Handling bookmark status request for status: {}", status_id);

    // TODO: Implement actual bookmark functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": status_id,
                "bookmarked": true
            },
            "error": null
        })),
    )
}

/// Unbookmark status handler
async fn unbookmark_status_handler(
    State(_state): State<AppState>,
    Path(status_id): Path<String>,
) -> impl IntoResponse {
    debug!(
        "Handling unbookmark status request for status: {}",
        status_id
    );

    // TODO: Implement actual unbookmark functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": status_id,
                "bookmarked": false
            },
            "error": null
        })),
    )
}

/// Vote poll handler
async fn vote_poll_handler(
    State(_state): State<AppState>,
    Path(poll_id): Path<String>,
) -> impl IntoResponse {
    debug!("Handling vote poll request for poll: {}", poll_id);

    // TODO: Implement actual poll voting functionality
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "id": poll_id,
                "expires_at": null,
                "expired": false,
                "multiple": false,
                "votes_count": 1,
                "voters_count": 1,
                "voted": true,
                "own_votes": [0],
                "options": [
                    {
                        "title": "Option 1",
                        "votes_count": 1
                    }
                ],
                "emojis": []
            },
            "error": null
        })),
    )
}

/// Trending tags handler
async fn trending_tags_handler() -> impl IntoResponse {
    debug!("Handling trending tags request");

    // TODO: Implement actual trending tags functionality
    Json(json!({
        "success": true,
        "data": [
            {
                "name": "rustodon",
                "url": "https://rustodon.example.com/tags/rustodon",
                "history": [
                    {
                        "day": "1577664000",
                        "uses": "1",
                        "accounts": "1"
                    }
                ]
            }
        ],
        "error": null
    }))
}

/// Trending statuses handler
async fn trending_statuses_handler() -> impl IntoResponse {
    debug!("Handling trending statuses request");

    // TODO: Implement actual trending statuses functionality
    Json(json!({
        "success": true,
        "data": [],
        "error": null
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
    debug!("Handling apps request");

    // TODO: Implement actual apps functionality
    Json(json!({
        "success": true,
        "data": [],
        "error": null
    }))
}

/// Followers handler
async fn followers_handler(
    State(_state): State<AppState>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    debug!("Handling followers request for account: {}", account_id);

    // TODO: Implement actual followers functionality
    Json(json!({
        "success": true,
        "data": [],
        "error": null
    }))
}

/// Following handler
async fn following_handler(
    State(_state): State<AppState>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    debug!("Handling following request for account: {}", account_id);

    // TODO: Implement actual following functionality
    Json(json!({
        "success": true,
        "data": [],
        "error": null
    }))
}

/// Accounts search handler
async fn accounts_search_handler() -> impl IntoResponse {
    debug!("Handling accounts search request");

    // TODO: Implement actual accounts search functionality
    Json(json!({
        "success": true,
        "data": [],
        "error": null
    }))
}

/// Get status handler
async fn get_status_handler(
    State(_state): State<AppState>,
    Path(status_id): Path<String>,
) -> impl IntoResponse {
    debug!("Handling get status request for status: {}", status_id);

    // TODO: Implement actual get status functionality
    Json(json!({
        "success": true,
        "data": {
            "id": status_id,
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
        },
        "error": null
    }))
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
        // Add assertions for the JSON response
    }

    #[tokio::test]
    async fn test_accounts_handler() {
        let response = accounts_handler().await;
        // Add assertions for the JSON response
    }

    #[tokio::test]
    async fn test_statuses_handler() {
        let response = statuses_handler().await;
        // Add assertions for the JSON response
    }

    #[tokio::test]
    async fn test_public_timeline_handler() {
        let response = public_timeline_handler().await;
        // Add assertions for the JSON response
    }

    #[tokio::test]
    async fn test_apps_handler() {
        let response = apps_handler().await;
        // Add assertions for the JSON response
    }
}
