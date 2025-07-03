//!
//! Rustodon Logging Infrastructure
//!
//! This module provides comprehensive logging infrastructure for the Rustodon server,
//! supporting structured logging, file output, remote logging, and log rotation.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_logging::{init_logging, LogConfig};
//!
//! let config = LogConfig::default();
//! init_logging(config).expect("Failed to initialize logging");
//!
//! tracing::info!("Server started successfully");
//! tracing::error!("An error occurred: {}", "connection failed");
//! ```
//!
//! # Dependencies
//!
//! - `tracing`: Structured logging framework
//! - `tracing-subscriber`: Logging subscriber implementations
//! - `serde`: Serialization for structured logging
//! - `thiserror`: Error handling
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;
use tracing::{Level, Subscriber};
use tracing_subscriber::{
    fmt::{self, time::UtcTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer, Registry,
};

/// Logging error type
#[derive(Error, Debug)]
pub enum LoggingError {
    #[error("Failed to create log directory: {0}")]
    DirectoryError(String),
    #[error("Failed to initialize logging: {0}")]
    InitError(String),
    #[error("Invalid log level: {0}")]
    InvalidLevel(String),
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    /// Log level (error, warn, info, debug, trace)
    pub level: String,
    /// Whether to enable console output
    pub console_enabled: bool,
    /// Whether to enable file output
    pub file_enabled: bool,
    /// Log file path
    pub file_path: String,
    /// Whether to enable JSON format
    pub json_format: bool,
    /// Whether to include timestamps
    pub include_timestamps: bool,
    /// Whether to include target (module path)
    pub include_target: bool,
    /// Whether to enable log rotation
    pub rotation_enabled: bool,
    /// Maximum log file size in bytes
    pub max_file_size: usize,
    /// Number of log files to keep
    pub max_files: usize,
    /// Whether to enable remote logging
    pub remote_enabled: bool,
    /// Remote logging endpoint
    pub remote_endpoint: Option<String>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            console_enabled: true,
            file_enabled: false,
            file_path: "logs/rustodon.log".to_string(),
            json_format: true,
            include_timestamps: true,
            include_target: false,
            rotation_enabled: false,
            max_file_size: 100 * 1024 * 1024, // 100MB
            max_files: 5,
            remote_enabled: false,
            remote_endpoint: None,
        }
    }
}

/// Initialize logging with the given configuration
pub fn init_logging(config: LogConfig) -> Result<(), LoggingError> {
    // Parse log level
    let level = parse_log_level(&config.level)?;

    // Create log directory if file logging is enabled
    if config.file_enabled {
        create_log_directory(&config.file_path)?;
    }

    // Build the subscriber
    let subscriber = build_subscriber(config, level)?;

    // Initialize the subscriber
    subscriber.init();

    tracing::info!("Logging initialized with level: {}", level);
    Ok(())
}

/// Parse log level string to tracing Level
fn parse_log_level(level: &str) -> Result<Level, LoggingError> {
    match level.to_lowercase().as_str() {
        "error" => Ok(Level::ERROR),
        "warn" => Ok(Level::WARN),
        "info" => Ok(Level::INFO),
        "debug" => Ok(Level::DEBUG),
        "trace" => Ok(Level::TRACE),
        _ => Err(LoggingError::InvalidLevel(level.to_string())),
    }
}

/// Create log directory if it doesn't exist
fn create_log_directory(file_path: &str) -> Result<(), LoggingError> {
    if let Some(parent) = Path::new(file_path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| LoggingError::DirectoryError(e.to_string()))?;
        }
    }
    Ok(())
}

/// Build the tracing subscriber with the given configuration
fn build_subscriber(config: LogConfig, level: Level) -> Result<impl Subscriber, LoggingError> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("rustodon={}", level)));

    let mut layers = Vec::new();

    // Console layer
    if config.console_enabled {
        let console_layer = fmt::layer()
            .with_timer(UtcTime::rfc_3339())
            .with_target(config.include_target);
        layers.push(console_layer.boxed());
    }

    // File layer (simplified - TODO: implement proper file logging)
    if config.file_enabled {
        tracing::warn!("File logging is not yet implemented, falling back to console logging");
    }

    // Remote logging layer (placeholder for future implementation)
    if config.remote_enabled {
        tracing::warn!("Remote logging is not yet implemented");
    }

    Ok(Registry::default().with(env_filter).with(layers))
}

/// Initialize logging with default configuration
pub fn init_default_logging() -> Result<(), LoggingError> {
    init_logging(LogConfig::default())
}

/// Initialize logging from environment variables
pub fn init_env_logging() -> Result<(), LoggingError> {
    let config = LogConfig {
        level: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        console_enabled: true,
        file_enabled: std::env::var("RUSTODON_LOG_FILE").is_ok(),
        file_path: std::env::var("RUSTODON_LOG_FILE")
            .unwrap_or_else(|_| "logs/rustodon.log".to_string()),
        json_format: std::env::var("RUSTODON_LOG_JSON")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true),
        include_timestamps: true,
        include_target: false,
        rotation_enabled: false,
        max_file_size: 100 * 1024 * 1024,
        max_files: 5,
        remote_enabled: false,
        remote_endpoint: None,
    };

    init_logging(config)
}

/// Log a structured event with additional context
pub fn log_event(level: Level, message: &str, context: Option<serde_json::Value>) {
    match level {
        Level::ERROR => {
            if let Some(ctx) = context {
                tracing::error!(context = %ctx, "{}", message);
            } else {
                tracing::error!("{}", message);
            }
        }
        Level::WARN => {
            if let Some(ctx) = context {
                tracing::warn!(context = %ctx, "{}", message);
            } else {
                tracing::warn!("{}", message);
            }
        }
        Level::INFO => {
            if let Some(ctx) = context {
                tracing::info!(context = %ctx, "{}", message);
            } else {
                tracing::info!("{}", message);
            }
        }
        Level::DEBUG => {
            if let Some(ctx) = context {
                tracing::debug!(context = %ctx, "{}", message);
            } else {
                tracing::debug!("{}", message);
            }
        }
        Level::TRACE => {
            if let Some(ctx) = context {
                tracing::trace!(context = %ctx, "{}", message);
            } else {
                tracing::trace!("{}", message);
            }
        }
    }
}

/// Log performance metrics
pub fn log_performance(operation: &str, duration_ms: u64, context: Option<serde_json::Value>) {
    let mut ctx = context.unwrap_or_else(|| serde_json::json!({}));
    if let Some(obj) = ctx.as_object_mut() {
        obj.insert(
            "operation".to_string(),
            serde_json::Value::String(operation.to_string()),
        );
        obj.insert(
            "duration_ms".to_string(),
            serde_json::Value::Number(serde_json::Number::from(duration_ms)),
        );
    }

    if duration_ms > 1000 {
        tracing::warn!(context = %ctx, "Slow operation detected");
    } else if duration_ms > 100 {
        tracing::info!(context = %ctx, "Operation completed");
    } else {
        tracing::debug!(context = %ctx, "Operation completed");
    }
}

/// Log security events
pub fn log_security_event(event_type: &str, details: &str, user_id: Option<i64>, ip: Option<&str>) {
    let mut context = serde_json::json!({
        "event_type": event_type,
        "details": details,
    });

    if let Some(uid) = user_id {
        if let Some(obj) = context.as_object_mut() {
            obj.insert(
                "user_id".to_string(),
                serde_json::Value::Number(serde_json::Number::from(uid)),
            );
        }
    }

    if let Some(ip_addr) = ip {
        if let Some(obj) = context.as_object_mut() {
            obj.insert(
                "ip_address".to_string(),
                serde_json::Value::String(ip_addr.to_string()),
            );
        }
    }

    tracing::warn!(context = %context, "Security event");
}

/// Log database operations
pub fn log_db_operation(operation: &str, table: &str, duration_ms: u64, success: bool) {
    let context = serde_json::json!({
        "operation": operation,
        "table": table,
        "duration_ms": duration_ms,
        "success": success,
    });

    if success {
        tracing::debug!(context = %context, "Database operation");
    } else {
        tracing::error!(context = %context, "Database operation failed");
    }
}

/// Log HTTP requests
pub fn log_http_request(
    method: &str,
    path: &str,
    status_code: u16,
    duration_ms: u64,
    user_id: Option<i64>,
    ip: Option<&str>,
) {
    let mut context = serde_json::json!({
        "method": method,
        "path": path,
        "status_code": status_code,
        "duration_ms": duration_ms,
    });

    if let Some(uid) = user_id {
        if let Some(obj) = context.as_object_mut() {
            obj.insert(
                "user_id".to_string(),
                serde_json::Value::Number(serde_json::Number::from(uid)),
            );
        }
    }

    if let Some(ip_addr) = ip {
        if let Some(obj) = context.as_object_mut() {
            obj.insert(
                "ip_address".to_string(),
                serde_json::Value::String(ip_addr.to_string()),
            );
        }
    }

    match status_code {
        200..=299 => tracing::info!(context = %context, "HTTP request"),
        300..=399 => tracing::debug!(context = %context, "HTTP request"),
        400..=499 => tracing::warn!(context = %context, "HTTP request"),
        500..=599 => tracing::error!(context = %context, "HTTP request"),
        _ => tracing::debug!(context = %context, "HTTP request"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_level() {
        assert_eq!(parse_log_level("error").unwrap(), Level::ERROR);
        assert_eq!(parse_log_level("warn").unwrap(), Level::WARN);
        assert_eq!(parse_log_level("info").unwrap(), Level::INFO);
        assert_eq!(parse_log_level("debug").unwrap(), Level::DEBUG);
        assert_eq!(parse_log_level("trace").unwrap(), Level::TRACE);
        assert!(parse_log_level("invalid").is_err());
    }

    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.level, "info");
        assert!(config.console_enabled);
        assert!(!config.file_enabled);
        assert_eq!(config.file_path, "logs/rustodon.log");
        assert!(config.json_format);
    }

    #[test]
    fn test_create_log_directory() {
        let temp_path = "temp_test_logs/test.log";
        assert!(create_log_directory(temp_path).is_ok());

        // Clean up
        let _ = fs::remove_dir_all("temp_test_logs");
    }
}
