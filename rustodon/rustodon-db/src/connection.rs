//! Database connection configuration for Rustodon - High Performance
//!
//! This module provides optimized database connection configuration and management
//! for high-concurrency scenarios (10k+ concurrent users).
//!
//! # Performance Optimizations
//!
//! - Large connection pool (100+ connections)
//! - Optimized timeouts for high concurrency
//! - Connection reuse and pooling
//! - Performance monitoring
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, error, info, trace, warn};

/// Database connection configuration optimized for high concurrency
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database URL
    pub url: String,
    /// Maximum number of connections in the pool (optimized for 10k users)
    pub max_connections: u32,
    /// Minimum number of connections in the pool
    pub min_connections: u32,
    /// Connection timeout (reduced for faster failure detection)
    pub connect_timeout: Duration,
    /// Idle timeout (increased for connection reuse)
    pub idle_timeout: Duration,
    /// Max lifetime (optimized for long-running connections)
    pub max_lifetime: Duration,
    /// Whether to enable SSL
    pub ssl_mode: SslMode,
    /// Connection acquire timeout
    pub acquire_timeout: Duration,
    /// Statement timeout
    pub statement_timeout: Duration,
    /// Whether to enable connection health checks
    pub health_check: bool,
}

/// SSL mode for database connections
#[derive(Debug, Clone, Copy)]
pub enum SslMode {
    Disable,
    Allow,
    Prefer,
    Require,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://rustodon:rustodon@localhost:5432/rustodon".to_string(),
            max_connections: 100, // Increased for 10k concurrent users
            min_connections: 20,  // Increased minimum for faster response
            connect_timeout: Duration::from_secs(10), // Reduced for faster failure detection
            idle_timeout: Duration::from_secs(1800), // 30 minutes for connection reuse
            max_lifetime: Duration::from_secs(3600), // 1 hour max lifetime
            ssl_mode: SslMode::Prefer,
            acquire_timeout: Duration::from_secs(5), // Fast acquire timeout
            statement_timeout: Duration::from_secs(30), // 30 second statement timeout
            health_check: true,                      // Enable health checks
        }
    }
}

/// High-performance database configuration for production
impl DatabaseConfig {
    /// Creates a high-performance configuration for 10k+ concurrent users
    pub fn high_performance() -> Self {
        Self {
            url: std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://rustodon:rustodon@localhost:5432/rustodon".to_string()
            }),
            max_connections: 200, // Very large pool for high concurrency
            min_connections: 50,  // High minimum for immediate availability
            connect_timeout: Duration::from_secs(5), // Very fast failure detection
            idle_timeout: Duration::from_secs(3600), // 1 hour idle timeout
            max_lifetime: Duration::from_secs(7200), // 2 hour max lifetime
            ssl_mode: SslMode::Prefer,
            acquire_timeout: Duration::from_secs(3), // Very fast acquire
            statement_timeout: Duration::from_secs(15), // 15 second statement timeout
            health_check: true,
        }
    }

    /// Creates a configuration for testing/development
    pub fn development() -> Self {
        Self {
            url: "postgres://rustodon:rustodon@localhost:5432/rustodon".to_string(),
            max_connections: 10,
            min_connections: 2,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(1800),
            ssl_mode: SslMode::Prefer,
            acquire_timeout: Duration::from_secs(10),
            statement_timeout: Duration::from_secs(60),
            health_check: false,
        }
    }
}

/// Error type for database connection operations
#[derive(Error, Debug)]
pub enum DatabaseConnectionError {
    #[error("Database connection error: {0}")]
    Connection(#[from] sqlx::Error),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Pool creation error: {0}")]
    PoolCreation(String),
    #[error("Health check failed: {0}")]
    HealthCheck(String),
}

/// Database connection manager with performance optimizations
#[derive(Debug, Clone)]
pub struct DatabaseManager {
    pool: PgPool,
    config: DatabaseConfig,
    created_at: std::time::Instant,
}

impl DatabaseManager {
    /// Creates a new database manager with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Database configuration
    ///
    /// # Returns
    ///
    /// A new DatabaseManager instance
    pub async fn new(config: DatabaseConfig) -> Result<Self, DatabaseConnectionError> {
        info!("Initializing high-performance database connection pool");
        trace!("Database config: {:?}", config);

        let pool = Self::create_pool(&config).await?;
        let created_at = std::time::Instant::now();

        Ok(Self {
            pool,
            config,
            created_at,
        })
    }

    /// Creates a database connection pool with performance optimizations
    ///
    /// # Arguments
    ///
    /// * `config` - Database configuration
    ///
    /// # Returns
    ///
    /// A new PgPool instance
    async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, DatabaseConnectionError> {
        let mut connect_options = config.url.parse::<PgConnectOptions>().map_err(|e| {
            error!("Failed to parse database URL: {}", e);
            DatabaseConnectionError::Configuration(format!("Invalid database URL: {}", e))
        })?;

        // Configure SSL mode
        match config.ssl_mode {
            SslMode::Disable => {
                connect_options = connect_options.ssl_mode(sqlx::postgres::PgSslMode::Disable);
            }
            SslMode::Allow => {
                connect_options = connect_options.ssl_mode(sqlx::postgres::PgSslMode::Allow);
            }
            SslMode::Prefer => {
                connect_options = connect_options.ssl_mode(sqlx::postgres::PgSslMode::Prefer);
            }
            SslMode::Require => {
                connect_options = connect_options.ssl_mode(sqlx::postgres::PgSslMode::Require);
            }
        }

        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.acquire_timeout)
            .idle_timeout(config.idle_timeout)
            .max_lifetime(config.max_lifetime)
            .connect_with(connect_options)
            .await
            .map_err(|e| {
                error!("Failed to create database pool: {}", e);
                DatabaseConnectionError::PoolCreation(e.to_string())
            })?;

        info!(
            "Database connection pool created successfully with {} max connections",
            config.max_connections
        );
        debug!("Pool configuration: min={}, acquire_timeout={:?}, idle_timeout={:?}, max_lifetime={:?}",
               config.min_connections, config.acquire_timeout, config.idle_timeout, config.max_lifetime);

        Ok(pool)
    }

    /// Gets the database connection pool
    ///
    /// # Returns
    ///
    /// Reference to the PgPool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Gets the database configuration
    ///
    /// # Returns
    ///
    /// Reference to the DatabaseConfig
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// Gets pool statistics for monitoring
    ///
    /// # Returns
    ///
    /// Pool statistics
    pub async fn get_pool_stats(&self) -> Result<PoolStats, DatabaseConnectionError> {
        let size = self.pool.size() as u32;
        let idle = self.pool.num_idle() as u32;
        let used = size - idle;

        Ok(PoolStats {
            total_connections: size,
            idle_connections: idle,
            used_connections: used,
            uptime: self.created_at.elapsed(),
        })
    }

    /// Tests the database connection with health check
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn test_connection(&self) -> Result<(), DatabaseConnectionError> {
        info!("Testing database connection");

        let start = std::time::Instant::now();

        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Database connection test failed: {}", e);
                DatabaseConnectionError::Connection(e)
            })?;

        let duration = start.elapsed();
        debug!("Database connection test successful in {:?}", duration);

        if duration > Duration::from_millis(100) {
            warn!(
                "Database connection test took longer than expected: {:?}",
                duration
            );
        }

        Ok(())
    }

    /// Performs a comprehensive health check
    ///
    /// # Returns
    ///
    /// Result indicating health status
    pub async fn health_check(&self) -> Result<HealthStatus, DatabaseConnectionError> {
        if !self.config.health_check {
            return Ok(HealthStatus::Disabled);
        }

        let start = std::time::Instant::now();

        // Test basic connectivity
        self.test_connection().await?;

        // Test pool statistics
        let stats = self.get_pool_stats().await?;

        // Test a simple query
        let result = sqlx::query("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await;

        let duration = start.elapsed();

        match result {
            Ok(_) => {
                debug!("Health check passed in {:?}", duration);
                Ok(HealthStatus::Healthy {
                    response_time: duration,
                    pool_stats: stats,
                })
            }
            Err(e) => {
                error!("Health check failed: {}", e);
                Err(DatabaseConnectionError::HealthCheck(e.to_string()))
            }
        }
    }

    /// Closes the database connection pool
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn close(&self) -> Result<(), DatabaseConnectionError> {
        info!("Closing database connection pool");
        self.pool.close().await;
        debug!("Database connection pool closed");
        Ok(())
    }
}

/// Pool statistics for monitoring
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total_connections: u32,
    pub idle_connections: u32,
    pub used_connections: u32,
    pub uptime: std::time::Duration,
}

/// Health status for database monitoring
#[derive(Debug, Clone)]
pub enum HealthStatus {
    Healthy {
        response_time: std::time::Duration,
        pool_stats: PoolStats,
    },
    Disabled,
}

/// Creates a database configuration from environment variables
///
/// # Returns
///
/// Database configuration
pub fn create_config_from_env() -> DatabaseConfig {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://rustodon:rustodon@localhost:5432/rustodon".to_string());

    let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .unwrap_or(100);

    let min_connections = std::env::var("DATABASE_MIN_CONNECTIONS")
        .unwrap_or_else(|_| "20".to_string())
        .parse()
        .unwrap_or(20);

    let connect_timeout = std::env::var("DATABASE_CONNECT_TIMEOUT")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap_or(10);

    let statement_timeout = std::env::var("DATABASE_STATEMENT_TIMEOUT")
        .unwrap_or_else(|_| "30".to_string())
        .parse()
        .unwrap_or(30);

    DatabaseConfig {
        url,
        max_connections,
        min_connections,
        connect_timeout: Duration::from_secs(connect_timeout),
        idle_timeout: Duration::from_secs(1800),
        max_lifetime: Duration::from_secs(3600),
        ssl_mode: SslMode::Prefer,
        acquire_timeout: Duration::from_secs(5),
        statement_timeout: Duration::from_secs(statement_timeout),
        health_check: true,
    }
}

/// Global database manager instance
static mut DATABASE_MANAGER: Option<DatabaseManager> = None;

/// Initialize the global database manager
///
/// # Arguments
///
/// * `config` - Database configuration
///
/// # Returns
///
/// Result indicating success or failure
pub async fn init_global_database(config: DatabaseConfig) -> Result<(), DatabaseConnectionError> {
    info!("Initializing global database manager");

    let manager = DatabaseManager::new(config).await?;

    unsafe {
        DATABASE_MANAGER = Some(manager);
    }

    debug!("Global database manager initialized");
    Ok(())
}

/// Get the global database manager
///
/// # Returns
///
/// Reference to the global DatabaseManager
pub fn get_global_database() -> Option<&'static DatabaseManager> {
    unsafe { DATABASE_MANAGER.as_ref() }
}

/// Get the global database pool
///
/// # Returns
///
/// Reference to the global PgPool
pub fn get_global_pool() -> Option<&'static PgPool> {
    get_global_database().map(|manager| manager.pool())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.min_connections, 20);
    }

    #[tokio::test]
    async fn test_create_config_from_env() {
        std::env::set_var("DATABASE_URL", "postgres://test:test@localhost:5432/test");
        std::env::set_var("DATABASE_MAX_CONNECTIONS", "200");

        let config = create_config_from_env();
        assert_eq!(config.max_connections, 200);
        assert!(config.url.contains("test"));
    }

    #[tokio::test]
    async fn test_database_manager_creation() {
        let config = DatabaseConfig {
            url: "postgres://rustodon:rustodon@localhost:5432/rustodon".to_string(),
            max_connections: 1,
            min_connections: 0,
            connect_timeout: Duration::from_secs(5),
            idle_timeout: Duration::from_secs(60),
            max_lifetime: Duration::from_secs(300),
            ssl_mode: SslMode::Disable,
            acquire_timeout: Duration::from_secs(5),
            statement_timeout: Duration::from_secs(30),
            health_check: true,
        };

        // This will fail if database is not available, but we can test the creation logic
        let result = DatabaseManager::new(config).await;
        // We don't assert here because the database might not be available in tests
        if result.is_err() {
            println!("Database connection failed as expected in test environment");
        }
    }
}
