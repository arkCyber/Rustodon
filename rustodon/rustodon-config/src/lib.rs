//!
//! Rustodon Configuration Management
//!
//! This module provides configuration management for the Rustodon server,
//! supporting environment variables, configuration files, and environment-specific settings.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_config::Config;
//!
//! let config = Config::load().expect("Failed to load configuration");
//! println!("Database URL: {}", config.database.url);
//! println!("Server port: {}", config.server.port);
//! ```
//!
//! # Dependencies
//!
//! - `serde`: Serialization/deserialization
//! - `thiserror`: Error handling
//! - `tracing`: Logging
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use thiserror::Error;
use tracing::{debug, error, info, warn};

/// Configuration error type
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Environment variable not found: {0}")]
    MissingEnvVar(String),
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
    #[error("Failed to read configuration file: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Failed to parse configuration: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Failed to parse TOML configuration: {0}")]
    TomlParseError(#[from] toml::de::Error),
    #[error("Configuration validation failed: {0}")]
    ValidationError(String),
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub timeout: u64,
    /// Whether to enable SQL query logging
    pub log_queries: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://rustodon:rustodon@localhost:5432/rustodon".to_string(),
            max_connections: 10,
            timeout: 30,
            log_queries: false,
        }
    }
}

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,
    /// Connection pool size
    pub pool_size: u32,
    /// Connection timeout in seconds
    pub timeout: u64,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            pool_size: 5,
            timeout: 5,
        }
    }
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server host to bind to
    pub host: String,
    /// Server port to bind to
    pub port: u16,
    /// Maximum request body size in bytes
    pub max_body_size: usize,
    /// Request timeout in seconds
    pub timeout: u64,
    /// Whether to enable CORS
    pub enable_cors: bool,
    /// CORS allowed origins
    pub cors_origins: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            max_body_size: 10 * 1024 * 1024, // 10MB
            timeout: 30,
            enable_cors: true,
            cors_origins: vec!["*".to_string()],
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (error, warn, info, debug, trace)
    pub level: String,
    /// Whether to enable structured logging
    pub structured: bool,
    /// Log format (json, text)
    pub format: String,
    /// Whether to include timestamps
    pub include_timestamps: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            structured: true,
            format: "json".to_string(),
            include_timestamps: true,
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Secret key for JWT tokens
    pub secret_key: String,
    /// JWT token expiration time in hours
    pub token_expiration: u64,
    /// Password minimum length
    pub min_password_length: usize,
    /// Whether to require strong passwords
    pub require_strong_passwords: bool,
    /// Rate limiting requests per minute
    pub rate_limit_per_minute: u32,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            secret_key: "your-secret-key-here".to_string(),
            token_expiration: 24,
            min_password_length: 8,
            require_strong_passwords: true,
            rate_limit_per_minute: 60,
        }
    }
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage backend type (local, s3, etc.)
    pub backend: String,
    /// Local storage path
    pub local_path: String,
    /// S3 bucket name (if using S3)
    pub s3_bucket: Option<String>,
    /// S3 region (if using S3)
    pub s3_region: Option<String>,
    /// Maximum file size in bytes
    pub max_file_size: usize,
    /// Allowed file types
    pub allowed_types: Vec<String>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend: "local".to_string(),
            local_path: "./storage".to_string(),
            s3_bucket: None,
            s3_region: None,
            max_file_size: 50 * 1024 * 1024, // 50MB
            allowed_types: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/gif".to_string(),
                "image/webp".to_string(),
            ],
        }
    }
}

/// Email configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    /// SMTP server host
    pub smtp_host: String,
    /// SMTP server port
    pub smtp_port: u16,
    /// SMTP username
    pub smtp_username: String,
    /// SMTP password
    pub smtp_password: String,
    /// Whether to use TLS
    pub use_tls: bool,
    /// From email address
    pub from_address: String,
    /// From name
    pub from_name: String,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            smtp_host: "localhost".to_string(),
            smtp_port: 587,
            smtp_username: "".to_string(),
            smtp_password: "".to_string(),
            use_tls: true,
            from_address: "noreply@rustodon.local".to_string(),
            from_name: "Rustodon".to_string(),
        }
    }
}

/// Federation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationConfig {
    /// Domain name for this instance
    pub domain: String,
    /// Whether federation is enabled
    pub enabled: bool,
    /// Maximum number of outgoing connections
    pub max_outgoing_connections: u32,
    /// Maximum number of incoming connections
    pub max_incoming_connections: u32,
    /// Whether to allow remote follows
    pub allow_remote_follows: bool,
}

impl Default for FederationConfig {
    fn default() -> Self {
        Self {
            domain: "rustodon.local".to_string(),
            enabled: true,
            max_outgoing_connections: 100,
            max_incoming_connections: 100,
            allow_remote_follows: true,
        }
    }
}

/// Main configuration struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Environment (development, staging, production)
    pub environment: String,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Redis configuration
    pub redis: RedisConfig,
    /// Server configuration
    pub server: ServerConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// Email configuration
    pub email: EmailConfig,
    /// Federation configuration
    pub federation: FederationConfig,
}

impl Config {
    /// Load configuration from environment variables and files
    pub fn load() -> Result<Self, ConfigError> {
        info!("Loading Rustodon configuration");

        // Get environment
        let environment = env::var("RUSTODON_ENV").unwrap_or_else(|_| "development".to_string());
        debug!("Environment: {}", environment);

        // Try to load from config file first
        let mut config = if let Ok(config) = Self::load_from_file(&environment) {
            info!("Loaded configuration from file");
            config
        } else {
            info!("No configuration file found, using defaults");
            Self::default()
        };

        // Override with environment variables
        config.override_from_env()?;

        // Validate configuration
        config.validate()?;

        info!("Configuration loaded successfully");
        debug!("Configuration: {:?}", config);

        Ok(config)
    }

    /// Load configuration from file
    fn load_from_file(environment: &str) -> Result<Self, ConfigError> {
        let config_paths = vec![
            format!("config/{}.json", environment),
            format!("config/{}.toml", environment),
            "config.json".to_string(),
            "config.toml".to_string(),
        ];

        for path in config_paths {
            if Path::new(&path).exists() {
                debug!("Loading configuration from {}", path);
                let content = fs::read_to_string(&path)?;

                if path.ends_with(".json") {
                    return Ok(serde_json::from_str(&content)?);
                } else if path.ends_with(".toml") {
                    return Ok(toml::from_str(&content)?);
                }
            }
        }

        Err(ConfigError::FileError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No configuration file found",
        )))
    }

    /// Override configuration with environment variables
    fn override_from_env(&mut self) -> Result<(), ConfigError> {
        // Database configuration
        if let Ok(url) = env::var("DATABASE_URL") {
            self.database.url = url;
        }
        if let Ok(max_conn) = env::var("DATABASE_MAX_CONNECTIONS") {
            self.database.max_connections = max_conn
                .parse()
                .map_err(|_| ConfigError::InvalidValue("DATABASE_MAX_CONNECTIONS".to_string()))?;
        }

        // Redis configuration
        if let Ok(url) = env::var("REDIS_URL") {
            self.redis.url = url;
        }

        // Server configuration
        if let Ok(host) = env::var("RUSTODON_HOST") {
            self.server.host = host;
        }
        if let Ok(port) = env::var("RUSTODON_PORT") {
            self.server.port = port
                .parse()
                .map_err(|_| ConfigError::InvalidValue("RUSTODON_PORT".to_string()))?;
        }

        // Logging configuration
        if let Ok(level) = env::var("RUST_LOG") {
            self.logging.level = level;
        }

        // Security configuration
        if let Ok(secret) = env::var("RUSTODON_SECRET_KEY") {
            self.security.secret_key = secret;
        }

        // Federation configuration
        if let Ok(domain) = env::var("RUSTODON_DOMAIN") {
            self.federation.domain = domain;
        }

        Ok(())
    }

    /// Validate configuration
    fn validate(&self) -> Result<(), ConfigError> {
        // Validate database URL
        if self.database.url.is_empty() {
            return Err(ConfigError::ValidationError(
                "Database URL cannot be empty".to_string(),
            ));
        }

        // Validate server configuration
        if self.server.port == 0 {
            return Err(ConfigError::ValidationError(
                "Server port cannot be 0".to_string(),
            ));
        }

        // Validate security configuration
        if self.security.secret_key == "your-secret-key-here" {
            warn!("Using default secret key. Please set RUSTODON_SECRET_KEY environment variable in production.");
        }

        // Validate federation configuration
        if self.federation.domain.is_empty() {
            return Err(ConfigError::ValidationError(
                "Federation domain cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Get configuration for a specific environment
    pub fn for_environment(environment: &str) -> Result<Self, ConfigError> {
        env::set_var("RUSTODON_ENV", environment);
        Self::load()
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    /// Get the full server URL
    pub fn server_url(&self) -> String {
        let protocol = if self.is_development() {
            "http"
        } else {
            "https"
        };
        format!("{}://{}:{}", protocol, self.server.host, self.server.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            environment: "development".to_string(),
            database: DatabaseConfig::default(),
            redis: RedisConfig::default(),
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
            security: SecurityConfig::default(),
            storage: StorageConfig::default(),
            email: EmailConfig::default(),
            federation: FederationConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.environment, "development");
        assert_eq!(config.server.port, 3000);
        assert_eq!(
            config.database.url,
            "postgres://rustodon:rustodon@localhost:5432/rustodon"
        );
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(config.validate().is_ok());

        // Test invalid port
        config.server.port = 0;
        assert!(config.validate().is_err());

        // Test empty database URL
        config.server.port = 3000;
        config.database.url = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_environment_checks() {
        let mut config = Config::default();

        config.environment = "development".to_string();
        assert!(config.is_development());
        assert!(!config.is_production());

        config.environment = "production".to_string();
        assert!(!config.is_development());
        assert!(config.is_production());
    }

    #[test]
    fn test_server_url() {
        let config = Config::default();
        assert_eq!(config.server_url(), "http://127.0.0.1:3000");

        let mut prod_config = Config::default();
        prod_config.environment = "production".to_string();
        assert_eq!(prod_config.server_url(), "https://127.0.0.1:3000");
    }
}
