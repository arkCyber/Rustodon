//! Unified error handling for Rustodon
//!
//! This module provides a comprehensive error handling system that unifies
//! all error types across the Rustodon project.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Unified error type for the entire Rustodon application
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum RustodonError {
    /// Database errors
    #[error("Database error: {0}")]
    Database(String),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Authorization errors
    #[error("Authorization error: {0}")]
    Authorization(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Not found errors
    #[error("Not found: {0}")]
    NotFound(String),

    /// Conflict errors
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Rate limit errors
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// Internal server errors
    #[error("Internal server error: {0}")]
    Internal(String),

    /// Bad request errors
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Unprocessable entity errors
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),

    /// Service unavailable errors
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    /// Network errors
    #[error("Network error: {0}")]
    Network(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Deserialization errors
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// File system errors
    #[error("File system error: {0}")]
    FileSystem(String),

    /// Media processing errors
    #[error("Media processing error: {0}")]
    MediaProcessing(String),

    /// Federation errors
    #[error("Federation error: {0}")]
    Federation(String),

    /// ActivityPub errors
    #[error("ActivityPub error: {0}")]
    ActivityPub(String),

    /// Cache errors
    #[error("Cache error: {0}")]
    Cache(String),

    /// Queue errors
    #[error("Queue error: {0}")]
    Queue(String),

    /// Search errors
    #[error("Search error: {0}")]
    Search(String),

    /// Email errors
    #[error("Email error: {0}")]
    Email(String),

    /// Webhook errors
    #[error("Webhook error: {0}")]
    Webhook(String),

    /// Worker errors
    #[error("Worker error: {0}")]
    Worker(String),

    /// Scheduler errors
    #[error("Scheduler error: {0}")]
    Scheduler(String),

    /// Admin errors
    #[error("Admin error: {0}")]
    Admin(String),

    /// CLI errors
    #[error("CLI error: {0}")]
    Cli(String),
}

impl RustodonError {
    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            RustodonError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::Authentication(_) => StatusCode::UNAUTHORIZED,
            RustodonError::Authorization(_) => StatusCode::FORBIDDEN,
            RustodonError::Validation(_) => StatusCode::BAD_REQUEST,
            RustodonError::NotFound(_) => StatusCode::NOT_FOUND,
            RustodonError::Conflict(_) => StatusCode::CONFLICT,
            RustodonError::RateLimit(_) => StatusCode::TOO_MANY_REQUESTS,
            RustodonError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::BadRequest(_) => StatusCode::BAD_REQUEST,
            RustodonError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            RustodonError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            RustodonError::Network(_) => StatusCode::BAD_GATEWAY,
            RustodonError::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::Serialization(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::Deserialization(_) => StatusCode::BAD_REQUEST,
            RustodonError::FileSystem(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::MediaProcessing(_) => StatusCode::UNPROCESSABLE_ENTITY,
            RustodonError::Federation(_) => StatusCode::BAD_GATEWAY,
            RustodonError::ActivityPub(_) => StatusCode::BAD_REQUEST,
            RustodonError::Cache(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::Queue(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::Search(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::Email(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::Webhook(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::Worker(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::Scheduler(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::Admin(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RustodonError::Cli(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get the error code for this error
    pub fn error_code(&self) -> &'static str {
        match self {
            RustodonError::Database(_) => "database_error",
            RustodonError::Authentication(_) => "authentication_error",
            RustodonError::Authorization(_) => "authorization_error",
            RustodonError::Validation(_) => "validation_error",
            RustodonError::NotFound(_) => "not_found",
            RustodonError::Conflict(_) => "conflict",
            RustodonError::RateLimit(_) => "rate_limit_exceeded",
            RustodonError::Internal(_) => "internal_error",
            RustodonError::BadRequest(_) => "bad_request",
            RustodonError::UnprocessableEntity(_) => "unprocessable_entity",
            RustodonError::ServiceUnavailable(_) => "service_unavailable",
            RustodonError::Network(_) => "network_error",
            RustodonError::Configuration(_) => "configuration_error",
            RustodonError::Serialization(_) => "serialization_error",
            RustodonError::Deserialization(_) => "deserialization_error",
            RustodonError::FileSystem(_) => "file_system_error",
            RustodonError::MediaProcessing(_) => "media_processing_error",
            RustodonError::Federation(_) => "federation_error",
            RustodonError::ActivityPub(_) => "activitypub_error",
            RustodonError::Cache(_) => "cache_error",
            RustodonError::Queue(_) => "queue_error",
            RustodonError::Search(_) => "search_error",
            RustodonError::Email(_) => "email_error",
            RustodonError::Webhook(_) => "webhook_error",
            RustodonError::Worker(_) => "worker_error",
            RustodonError::Scheduler(_) => "scheduler_error",
            RustodonError::Admin(_) => "admin_error",
            RustodonError::Cli(_) => "cli_error",
        }
    }

    /// Check if this error should be logged
    pub fn should_log(&self) -> bool {
        match self {
            RustodonError::Database(_) => true,
            RustodonError::Internal(_) => true,
            RustodonError::Configuration(_) => true,
            RustodonError::FileSystem(_) => true,
            RustodonError::Cache(_) => true,
            RustodonError::Queue(_) => true,
            RustodonError::Search(_) => true,
            RustodonError::Email(_) => true,
            RustodonError::Webhook(_) => true,
            RustodonError::Worker(_) => true,
            RustodonError::Scheduler(_) => true,
            RustodonError::Admin(_) => true,
            RustodonError::Cli(_) => true,
            _ => false,
        }
    }

    /// Check if this error should be reported to monitoring
    pub fn should_report(&self) -> bool {
        match self {
            RustodonError::Database(_) => true,
            RustodonError::Internal(_) => true,
            RustodonError::Configuration(_) => true,
            RustodonError::FileSystem(_) => true,
            RustodonError::Cache(_) => true,
            RustodonError::Queue(_) => true,
            RustodonError::Search(_) => true,
            RustodonError::Email(_) => true,
            RustodonError::Webhook(_) => true,
            RustodonError::Worker(_) => true,
            RustodonError::Scheduler(_) => true,
            RustodonError::Admin(_) => true,
            RustodonError::Cli(_) => true,
            _ => false,
        }
    }
}

/// Error response for API endpoints
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub error_code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl From<RustodonError> for ErrorResponse {
    fn from(error: RustodonError) -> Self {
        ErrorResponse {
            error: error.error_code().to_string(),
            error_code: error.error_code().to_string(),
            message: error.to_string(),
            details: None,
        }
    }
}

/// Result type for Rustodon operations
pub type RustodonResult<T> = Result<T, RustodonError>;

/// Convert RustodonError to HTTP response
impl IntoResponse for RustodonError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_response = ErrorResponse::from(self.clone());

        // Log the error if needed
        if self.should_log() {
            tracing::error!("API Error: {:?}", self);
        }

        (status, Json(error_response)).into_response()
    }
}

/// Error conversion traits
impl From<sqlx::Error> for RustodonError {
    fn from(error: sqlx::Error) -> Self {
        RustodonError::Database(error.to_string())
    }
}

impl From<serde_json::Error> for RustodonError {
    fn from(error: serde_json::Error) -> Self {
        RustodonError::Serialization(error.to_string())
    }
}

impl From<std::io::Error> for RustodonError {
    fn from(error: std::io::Error) -> Self {
        RustodonError::FileSystem(error.to_string())
    }
}

impl From<reqwest::Error> for RustodonError {
    fn from(error: reqwest::Error) -> Self {
        RustodonError::Network(error.to_string())
    }
}

impl From<tokio::task::JoinError> for RustodonError {
    fn from(error: tokio::task::JoinError) -> Self {
        RustodonError::Worker(error.to_string())
    }
}

impl From<redis::RedisError> for RustodonError {
    fn from(error: redis::RedisError) -> Self {
        RustodonError::Cache(error.to_string())
    }
}

/// Error context for adding additional information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub resource: Option<String>,
    pub user_id: Option<i64>,
    pub request_id: Option<String>,
}

impl ErrorContext {
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            resource: None,
            user_id: None,
            request_id: None,
        }
    }

    pub fn with_resource(mut self, resource: impl Into<String>) -> Self {
        self.resource = Some(resource.into());
        self
    }

    pub fn with_user_id(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
}

/// Error with context
#[derive(Debug, Clone)]
pub struct ContextualError {
    pub error: RustodonError,
    pub context: ErrorContext,
}

impl ContextualError {
    pub fn new(error: RustodonError, context: ErrorContext) -> Self {
        Self { error, context }
    }

    pub fn with_context(error: RustodonError, operation: impl Into<String>) -> Self {
        Self {
            error,
            context: ErrorContext::new(operation),
        }
    }
}

impl fmt::Display for ContextualError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (operation: {})", self.error, self.context.operation)
    }
}

impl std::error::Error for ContextualError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(
            RustodonError::NotFound("test".to_string()).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            RustodonError::BadRequest("test".to_string()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            RustodonError::Internal("test".to_string()).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(
            RustodonError::NotFound("test".to_string()).error_code(),
            "not_found"
        );
        assert_eq!(
            RustodonError::BadRequest("test".to_string()).error_code(),
            "bad_request"
        );
        assert_eq!(
            RustodonError::Internal("test".to_string()).error_code(),
            "internal_error"
        );
    }

    #[test]
    fn test_error_logging() {
        assert!(RustodonError::Database("test".to_string()).should_log());
        assert!(!RustodonError::NotFound("test".to_string()).should_log());
    }

    #[test]
    fn test_error_response() {
        let error = RustodonError::NotFound("User not found".to_string());
        let response = ErrorResponse::from(error);
        assert_eq!(response.error_code, "not_found");
        assert!(response.message.contains("User not found"));
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("create_user")
            .with_resource("users")
            .with_user_id(123)
            .with_request_id("req-456");

        assert_eq!(context.operation, "create_user");
        assert_eq!(context.resource, Some("users".to_string()));
        assert_eq!(context.user_id, Some(123));
        assert_eq!(context.request_id, Some("req-456".to_string()));
    }
}
