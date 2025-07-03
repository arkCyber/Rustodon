//! Configuration management for Rustodon
//!
//! This module provides configuration management functionality.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, trace};

/// Configuration struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Environment (development, staging, production)
    pub environment: String,
    /// Database URL
    pub database_url: String,
    /// Redis URL
    pub redis_url: String,
    /// Server port
    pub port: u16,
    /// Additional settings
    pub settings: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            environment: "development".to_string(),
            database_url: "postgresql://rustodon:rustodon@localhost:5432/rustodon".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            port: 3000,
            settings: HashMap::new(),
        }
    }
}

impl Config {
    /// Creates a new configuration
    pub fn new() -> Self {
        trace!("Creating new configuration");
        Self::default()
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        info!("Loading configuration from environment");

        let mut config = Self::default();

        if let Ok(env) = std::env::var("ENVIRONMENT") {
            config.environment = env;
        }

        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            config.database_url = db_url;
        }

        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            config.redis_url = redis_url;
        }

        if let Ok(port) = std::env::var("PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.port = port_num;
            }
        }

        debug!("Configuration loaded: {:?}", config);
        config
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.environment, "development");
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn test_config_new() {
        let config = Config::new();
        assert_eq!(config.environment, "development");
    }

    #[test]
    fn test_config_environment_checks() {
        let mut config = Config::default();
        assert!(config.is_development());
        assert!(!config.is_production());

        config.environment = "production".to_string();
        assert!(!config.is_development());
        assert!(config.is_production());
    }
}
