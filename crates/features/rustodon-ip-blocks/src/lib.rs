//! IP blocking functionality for Rustodon
//!
//! This module provides IP blocking capabilities for the Rustodon server.
//! It handles IP address blocking, CIDR range blocking, and IP-based access control.
//!
//! # Features
//!
//! - IP address blocking and unblocking
//! - CIDR range blocking
//! - Temporary and permanent blocks
//! - IP block management and querying
//! - Integration with request middleware
//!
//! # Examples
//!
//! ```rust
//! use rustodon_ip_blocks::IpBlockService;
//!
//! let service = IpBlockService::new(pool);
//! service.block_ip("192.168.1.1", "Spam", None).await?;
//! ```
//!
//! # Dependencies
//!
//! - `rustodon_core`: Core types and traits
//! - `rustodon_db`: Database operations
//! - `rustodon_config`: Configuration management
//! - `rustodon_logging`: Logging infrastructure
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, error, info, trace};

pub mod error;
pub mod models;
pub mod service;

pub use error::*;
pub use models::*;
pub use service::*;

/// IP block severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpBlockSeverity {
    /// No action taken, just logging
    Noop,
    /// Suspend the account
    Suspend,
    /// Silence the account
    Silence,
    /// Block the IP address
    Block,
}

impl std::fmt::Display for IpBlockSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpBlockSeverity::Noop => write!(f, "noop"),
            IpBlockSeverity::Suspend => write!(f, "suspend"),
            IpBlockSeverity::Silence => write!(f, "silence"),
            IpBlockSeverity::Block => write!(f, "block"),
        }
    }
}

impl Default for IpBlockSeverity {
    fn default() -> Self {
        Self::Block
    }
}

/// IP block configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBlockConfig {
    /// Whether IP blocking is enabled
    pub enabled: bool,
    /// Default severity for new blocks
    pub default_severity: IpBlockSeverity,
    /// Maximum number of failed attempts before blocking
    pub max_failed_attempts: u32,
    /// Block duration in seconds (None for permanent)
    pub block_duration: Option<u64>,
}

impl Default for IpBlockConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_severity: IpBlockSeverity::Block,
            max_failed_attempts: 5,
            block_duration: Some(3600), // 1 hour
        }
    }
}

/// Initialize IP blocking functionality
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `config` - IP blocking configuration
///
/// # Returns
///
/// Result indicating success or failure
pub async fn init_ip_blocks(pool: &PgPool, config: &IpBlockConfig) -> Result<(), IpBlockError> {
    info!("Initializing IP blocking functionality");
    trace!("IP block config: {:?}", config);

    // Create tables if they don't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ip_blocks (
            id BIGSERIAL PRIMARY KEY,
            ip_address INET NOT NULL,
            cidr_range CIDR,
            severity VARCHAR(20) NOT NULL DEFAULT 'block',
            reason TEXT,
            expires_at TIMESTAMP WITH TIME ZONE,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Failed to create ip_blocks table: {}", e);
        IpBlockError::Database(e)
    })?;

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_ip_blocks_ip_address ON ip_blocks(ip_address)")
        .execute(pool)
        .await
        .map_err(|e| {
            error!("Failed to create ip_blocks index: {}", e);
            IpBlockError::Database(e)
        })?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_ip_blocks_expires_at ON ip_blocks(expires_at)")
        .execute(pool)
        .await
        .map_err(|e| {
            error!("Failed to create ip_blocks expires index: {}", e);
            IpBlockError::Database(e)
        })?;

    debug!("IP blocking functionality initialized successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ip_block_config_default() {
        let config = IpBlockConfig::default();
        assert!(config.enabled);
        assert_eq!(config.default_severity, IpBlockSeverity::Block);
        assert_eq!(config.max_failed_attempts, 5);
        assert_eq!(config.block_duration, Some(3600));
    }

    #[tokio::test]
    async fn test_ip_block_severity_default() {
        assert_eq!(IpBlockSeverity::default(), IpBlockSeverity::Block);
    }

    #[tokio::test]
    async fn test_ip_block_severity_display() {
        assert_eq!(IpBlockSeverity::Noop.to_string(), "noop");
        assert_eq!(IpBlockSeverity::Suspend.to_string(), "suspend");
        assert_eq!(IpBlockSeverity::Silence.to_string(), "silence");
        assert_eq!(IpBlockSeverity::Block.to_string(), "block");
    }
}
