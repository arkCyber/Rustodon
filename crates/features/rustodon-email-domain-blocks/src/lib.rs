//! Email domain blocking functionality for Rustodon
//!
//! This module provides email domain blocking capabilities for the Rustodon server.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, info};

/// Email domain block model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDomainBlock {
    pub id: i64,
    pub domain: String,
    pub reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Error type for email domain blocking operations
#[derive(Error, Debug)]
pub enum EmailDomainBlockError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Invalid domain: {0}")]
    InvalidDomain(String),
}

/// Initialize email domain blocking functionality
pub async fn init_email_domain_blocks() -> Result<(), EmailDomainBlockError> {
    info!("Initializing email domain blocking functionality");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_domain_block() {
        let block = EmailDomainBlock {
            id: 1,
            domain: "example.com".to_string(),
            reason: Some("Spam".to_string()),
            created_at: chrono::Utc::now(),
        };
        assert_eq!(block.domain, "example.com");
    }
}
