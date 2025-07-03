//! Domain blocks module for Rustodon
//!
//! This module provides domain block functionality for the Rustodon server.
//! It handles blocking and unblocking domains, and querying blocked domains.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_domains::DomainBlock;
//!
//! let block = DomainBlock::create(&pool, user_id, "example.com").await?;
//! ```
//!
//! # Dependencies
//!
//! - `sqlx`: Database queries
//! - `serde`: Serialization
//! - `chrono`: DateTime handling
//! - `thiserror`: Error handling
//! - `tracing`: Logging
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;
use tracing::{error, info, trace};

/// Custom error type for domain blocks
#[derive(Error, Debug)]
pub enum DomainBlockError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Domain block not found")]
    NotFound,
    #[error("Already blocked")]
    AlreadyBlocked,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Domain block data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBlock {
    /// Unique identifier for the domain block
    pub id: i64,
    /// ID of the account that blocked the domain
    pub account_id: i64,
    /// The blocked domain
    pub domain: String,
    /// When the block was created
    pub created_at: DateTime<Utc>,
}

impl DomainBlock {
    /// Creates a new domain block
    pub async fn create(
        pool: &PgPool,
        account_id: i64,
        domain: &str,
    ) -> Result<Self, DomainBlockError> {
        trace!(
            "Creating domain block for account {} on domain {}",
            account_id,
            domain
        );

        // Check if already blocked
        let exists = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM domain_blocks WHERE account_id = $1 AND domain = $2"#,
            account_id,
            domain
        )
        .fetch_one(pool)
        .await?
        .count;
        if exists.unwrap_or(0) > 0 {
            return Err(DomainBlockError::AlreadyBlocked);
        }

        // Insert block
        let row = sqlx::query!(
            r#"INSERT INTO domain_blocks (account_id, domain) VALUES ($1, $2) RETURNING id, account_id, domain, created_at"#,
            account_id,
            domain
        )
        .fetch_one(pool)
        .await?;

        let block = DomainBlock {
            id: row.id,
            account_id: row.account_id,
            domain: row.domain,
            created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
        };
        info!(
            "Created domain block with id: {} for account {} on domain {}",
            block.id, account_id, domain
        );
        Ok(block)
    }

    /// Deletes a domain block
    pub async fn delete(
        pool: &PgPool,
        account_id: i64,
        domain: &str,
    ) -> Result<(), DomainBlockError> {
        trace!(
            "Deleting domain block for account {} on domain {}",
            account_id,
            domain
        );
        let result = sqlx::query!(
            r#"DELETE FROM domain_blocks WHERE account_id = $1 AND domain = $2"#,
            account_id,
            domain
        )
        .execute(pool)
        .await?;
        if result.rows_affected() == 0 {
            return Err(DomainBlockError::NotFound);
        }
        info!(
            "Deleted domain block for account {} on domain {}",
            account_id, domain
        );
        Ok(())
    }

    /// Checks if a domain is blocked by a user
    pub async fn exists(
        pool: &PgPool,
        account_id: i64,
        domain: &str,
    ) -> Result<bool, DomainBlockError> {
        trace!(
            "Checking if account {} has blocked domain {}",
            account_id,
            domain
        );
        let count = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM domain_blocks WHERE account_id = $1 AND domain = $2"#,
            account_id,
            domain
        )
        .fetch_one(pool)
        .await?
        .count;
        Ok(count.unwrap_or(0) > 0)
    }

    /// Gets all blocked domains for a user (with pagination)
    pub async fn get_blocked_domains(
        pool: &PgPool,
        account_id: i64,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Self>, DomainBlockError> {
        trace!("Getting blocked domains for account {}", account_id);
        let limit = limit.unwrap_or(20).min(80);
        let offset = offset.unwrap_or(0);
        let rows = sqlx::query!(
            r#"SELECT id, account_id, domain, created_at FROM domain_blocks WHERE account_id = $1 ORDER BY id DESC LIMIT $2 OFFSET $3"#,
            account_id,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;
        let blocks = rows
            .into_iter()
            .map(|row| DomainBlock {
                id: row.id,
                account_id: row.account_id,
                domain: row.domain,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
            })
            .collect();
        Ok(blocks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_domain_block_struct() {
        let block = DomainBlock {
            id: 1,
            account_id: 1,
            domain: "example.com".to_string(),
            created_at: Utc::now(),
        };
        assert_eq!(block.account_id, 1);
        assert_eq!(block.domain, "example.com");
    }
}
