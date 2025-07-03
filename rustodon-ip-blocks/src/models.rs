//! IP block models for Rustodon
//!
//! This module defines the data models for IP blocking functionality.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::{DateTime, NaiveDateTime, Utc};
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::str::FromStr;
use tracing::{debug, trace};

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
    pub reason: Option<String>,
    /// When the block expires (None for permanent)
    pub expires_at: Option<NaiveDateTime>,
    /// When the block was created
    pub created_at: NaiveDateTime,
    /// When the block was last updated
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
    /// * `expires_at` - When the block expires
    ///
    /// # Returns
    ///
    /// A new IpBlock instance
    pub fn new(
        ip_address: impl Into<String>,
        severity: IpBlockSeverity,
        reason: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        let ip_address = ip_address
            .into()
            .parse::<IpNetwork>()
            .unwrap_or_else(|_| "127.0.0.1".parse::<IpNetwork>().unwrap());
        let severity = severity.to_string();
        let now = Utc::now().naive_utc();

        trace!("Creating new IP block for: {}", ip_address);

        Self {
            id: 0, // Will be set by database
            ip_address,
            cidr_range: None,
            severity,
            reason,
            expires_at: expires_at.map(|dt| dt.naive_utc()),
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the block is expired
    ///
    /// # Returns
    ///
    /// True if the block is expired, false otherwise
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now().naive_utc() > expires_at
        } else {
            false // Permanent block
        }
    }

    /// Get the severity as an enum
    ///
    /// # Returns
    ///
    /// The severity as IpBlockSeverity enum
    pub fn severity_enum(&self) -> Result<IpBlockSeverity, IpBlockError> {
        match self.severity.as_str() {
            "noop" => Ok(IpBlockSeverity::Noop),
            "suspend" => Ok(IpBlockSeverity::Suspend),
            "silence" => Ok(IpBlockSeverity::Silence),
            "block" => Ok(IpBlockSeverity::Block),
            _ => Err(IpBlockError::InvalidSeverity(self.severity.clone())),
        }
    }

    /// Check if an IP address matches this block
    ///
    /// # Arguments
    ///
    /// * `ip` - IP address to check
    ///
    /// # Returns
    ///
    /// True if the IP matches this block
    pub fn matches_ip(&self, ip: &str) -> bool {
        if let Ok(ip_parsed) = ip.parse::<IpNetwork>() {
            if self.ip_address == ip_parsed {
                return true;
            }
        }

        // Check CIDR range if present
        if let Some(ref cidr) = self.cidr_range {
            if let Ok(ip_parsed) = ip.parse::<std::net::IpAddr>() {
                return cidr.contains(ip_parsed);
            }
        }

        false
    }
}

/// Create IP block request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIpBlockRequest {
    /// IP address to block
    pub ip_address: String,
    /// CIDR range (optional)
    pub cidr_range: Option<String>,
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
    use chrono::Duration;

    #[test]
    fn test_ip_block_new() {
        let block = IpBlock::new(
            "192.168.1.1",
            IpBlockSeverity::Block,
            Some("Spam".to_string()),
            Some(Utc::now() + Duration::hours(1)),
        );

        assert_eq!(block.ip_address.to_string(), "192.168.1.1/32");
        assert_eq!(block.severity, "block");
        assert_eq!(block.reason, Some("Spam".to_string()));
        assert!(!block.is_expired());
    }

    #[test]
    fn test_ip_block_expired() {
        let block = IpBlock::new(
            "192.168.1.1",
            IpBlockSeverity::Block,
            None,
            Some(Utc::now() - Duration::hours(1)),
        );

        assert!(block.is_expired());
    }

    #[test]
    fn test_ip_block_permanent() {
        let block = IpBlock::new("192.168.1.1", IpBlockSeverity::Block, None, None);

        assert!(!block.is_expired());
    }

    #[test]
    fn test_ip_block_severity_enum() {
        let mut block = IpBlock::new("192.168.1.1", IpBlockSeverity::Block, None, None);

        assert_eq!(block.severity_enum().unwrap(), IpBlockSeverity::Block);

        block.severity = "suspend".to_string();
        assert_eq!(block.severity_enum().unwrap(), IpBlockSeverity::Suspend);

        block.severity = "invalid".to_string();
        assert!(block.severity_enum().is_err());
    }

    #[test]
    fn test_ip_block_matches_ip() {
        let block = IpBlock::new("192.168.1.1", IpBlockSeverity::Block, None, None);

        assert!(block.matches_ip("192.168.1.1"));
        assert!(!block.matches_ip("192.168.1.2"));
    }
}
