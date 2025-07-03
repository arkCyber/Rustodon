//! IP block models for Rustodon
//!
//! This module defines the data models for IP blocking functionality.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::FromRow;
use tracing::trace;

use super::{IpBlockError, IpBlockSeverity};

/// IP block model representing a blocked IP address or range
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IpBlock {
    /// Unique identifier
    pub id: i64,
    /// IP address (IPv4 or IPv6)
    pub ip_address: IpNetwork,
    /// CIDR range (optional)
    pub cidr_range: Option<IpNetwork>,
    /// Block severity level
    pub severity: String,
    /// Reason for blocking
    pub reason: String,
    /// Expiration time (optional)
    pub expires_at: Option<NaiveDateTime>,
    /// Creation timestamp
    pub created_at: NaiveDateTime,
    /// Last update timestamp
    pub updated_at: NaiveDateTime,
}

impl IpBlock {
    /// Creates a new IP block
    ///
    /// # Arguments
    ///
    /// * `ip_address` - IP address to block
    /// * `severity` - Block severity
    /// * `reason` - Reason for blocking
    /// * `expires_at` - Optional expiration time
    ///
    /// # Returns
    ///
    /// A new IpBlock instance
    pub fn new(
        ip_address: IpNetwork,
        severity: IpBlockSeverity,
        reason: String,
        expires_at: Option<NaiveDateTime>,
    ) -> Self {
        trace!("Creating new IP block for: {}", ip_address);

        let severity = severity.to_string();
        let now = Utc::now().naive_utc();

        Self {
            id: 0, // Will be set by database
            ip_address,
            cidr_range: None,
            severity,
            reason,
            expires_at,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the block has expired
    ///
    /// # Returns
    ///
    /// True if the block has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now().naive_utc() > expires_at
        } else {
            false
        }
    }

    /// Get the severity level
    ///
    /// # Returns
    ///
    /// The severity level
    pub fn severity(&self) -> Result<IpBlockSeverity, IpBlockError> {
        match self.severity.as_str() {
            "noop" => Ok(IpBlockSeverity::Noop),
            "suspend" => Ok(IpBlockSeverity::Suspend),
            "silence" => Ok(IpBlockSeverity::Silence),
            "block" => Ok(IpBlockSeverity::Block),
            _ => Err(IpBlockError::Validation(format!(
                "Invalid severity: {}",
                self.severity
            ))),
        }
    }
}

/// Create IP block request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIpBlockRequest {
    /// IP address to block
    pub ip_address: IpNetwork,
    /// CIDR range (optional)
    pub cidr_range: Option<IpNetwork>,
    /// Block severity
    pub severity: IpBlockSeverity,
    /// Reason for blocking
    pub reason: Option<String>,
    /// Duration in seconds (None for permanent)
    pub duration: Option<u64>,
}

/// Update IP block request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIpBlockRequest {
    /// Block severity
    pub severity: Option<IpBlockSeverity>,
    /// Reason for blocking
    pub reason: Option<String>,
    /// Duration in seconds (None for permanent)
    pub duration: Option<u64>,
}

/// IP block query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBlockQuery {
    /// IP address to search for
    pub ip_address: Option<String>,
    /// Severity filter
    pub severity: Option<IpBlockSeverity>,
    /// Include expired blocks
    pub include_expired: bool,
    /// Page number
    pub page: Option<i64>,
    /// Page size
    pub limit: Option<i64>,
}

impl Default for IpBlockQuery {
    fn default() -> Self {
        Self {
            ip_address: None,
            severity: None,
            include_expired: false,
            page: Some(1),
            limit: Some(20),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_block_new() {
        let ip_address = "192.168.1.1".parse::<IpNetwork>().unwrap();
        let block = IpBlock::new(
            ip_address,
            IpBlockSeverity::Block,
            "Test block".to_string(),
            None,
        );
        assert_eq!(block.ip_address.to_string(), "192.168.1.1/32");
        assert_eq!(block.severity, "block");
        assert_eq!(block.reason, "Test block");
        assert!(!block.is_expired());
    }

    #[test]
    fn test_ip_block_severity_display() {
        assert_eq!(IpBlockSeverity::Noop.to_string(), "noop");
        assert_eq!(IpBlockSeverity::Suspend.to_string(), "suspend");
        assert_eq!(IpBlockSeverity::Silence.to_string(), "silence");
        assert_eq!(IpBlockSeverity::Block.to_string(), "block");
    }
}
