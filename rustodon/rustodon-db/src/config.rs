//! Database configuration for Rustodon
//!
//! This module handles database connection pool configuration.
//! It provides default configurations and connection management.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use crate::error::DbError;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::{info, trace};

/// Database connection pool configuration
#[derive(Debug)]
pub struct DbConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: std::time::Duration,
    pub idle_timeout: std::time::Duration,
    pub max_lifetime: std::time::Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: "postgres://localhost/rustodon".to_string(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout: std::time::Duration::from_secs(30),
            idle_timeout: std::time::Duration::from_secs(600),
            max_lifetime: std::time::Duration::from_secs(1800),
        }
    }
}

/// Create a new database connection pool
pub async fn create_pool(config: &DbConfig) -> Result<PgPool, DbError> {
    trace!("Creating database pool with config: {:?}", config);

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect(&config.url)
        .await?;

    info!("Database pool created successfully");
    Ok(pool)
}

/// Establish database connection (alias for create_pool with default config)
pub async fn establish_connection(database_url: &str) -> Result<PgPool, DbError> {
    let config = DbConfig {
        url: database_url.to_string(),
        ..Default::default()
    };
    create_pool(&config).await
}
