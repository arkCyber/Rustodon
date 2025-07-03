//! DomainBlock model for Rustodon
//!
//! This module defines the DomainBlock model and its database operations.
//! It handles domain blocking relationships.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use crate::error::DbError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info, trace};

/// DomainBlock model representing a user blocking a domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBlock {
    pub id: i64,
    pub account_id: i64,
    pub domain: String,
    pub created_at: Option<NaiveDateTime>,
}

impl DomainBlock {
    /// Get all domain blocks
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        trace!("Fetching all domain blocks");
        let domain_blocks = sqlx::query_as!(
            DomainBlock,
            "SELECT id, account_id, domain, created_at FROM domain_blocks ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} domain blocks", domain_blocks.len());
        Ok(domain_blocks)
    }

    /// Get domain blocks by account
    pub async fn get_by_account(pool: &PgPool, account_id: i64) -> Result<Vec<Self>, DbError> {
        trace!("Fetching domain blocks for account: {}", account_id);
        let domain_blocks = sqlx::query_as!(
            DomainBlock,
            "SELECT id, account_id, domain, created_at FROM domain_blocks WHERE account_id = $1 ORDER BY created_at DESC",
            account_id
        )
        .fetch_all(pool)
        .await?;

        info!(
            "Fetched {} domain blocks for account: {}",
            domain_blocks.len(),
            account_id
        );
        Ok(domain_blocks)
    }

    /// Create a new domain block
    pub async fn create(pool: &PgPool, account_id: i64, domain: &str) -> Result<Self, DbError> {
        trace!(
            "Creating domain block: {} blocks domain {}",
            account_id,
            domain
        );
        let domain_block = sqlx::query_as!(
            DomainBlock,
            "INSERT INTO domain_blocks (account_id, domain) VALUES ($1, $2) RETURNING id, account_id, domain, created_at",
            account_id,
            domain
        )
        .fetch_one(pool)
        .await?;

        info!(
            "Created domain block: {} blocks domain {}",
            account_id, domain
        );
        Ok(domain_block)
    }

    /// Remove a domain block
    pub async fn delete(pool: &PgPool, account_id: i64, domain: &str) -> Result<bool, DbError> {
        trace!(
            "Removing domain block: {} unblocks domain {}",
            account_id,
            domain
        );
        let result = sqlx::query!(
            "DELETE FROM domain_blocks WHERE account_id = $1 AND domain = $2",
            account_id,
            domain
        )
        .execute(pool)
        .await?;

        let removed = result.rows_affected() > 0;
        if removed {
            info!(
                "Removed domain block: {} unblocks domain {}",
                account_id, domain
            );
        } else {
            debug!(
                "Domain block not found for removal: {} -> {}",
                account_id, domain
            );
        }
        Ok(removed)
    }

    /// Check if a domain block exists
    pub async fn exists(pool: &PgPool, account_id: i64, domain: &str) -> Result<bool, DbError> {
        trace!(
            "Checking if domain block exists: {} -> {}",
            account_id,
            domain
        );
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM domain_blocks WHERE account_id = $1 AND domain = $2",
            account_id,
            domain
        )
        .fetch_one(pool)
        .await?;

        let exists = count.count.unwrap_or(0) > 0;
        debug!(
            "Domain block exists: {} -> {} = {}",
            account_id, domain, exists
        );
        Ok(exists)
    }

    /// Get blocked domains for an account with pagination
    pub async fn get_blocked_domains(
        pool: &PgPool,
        account_id: i64,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<String>, DbError> {
        trace!(
            "Fetching blocked domains for account: {} with limit: {:?}, offset: {:?}",
            account_id,
            limit,
            offset
        );

        let limit = limit.unwrap_or(40).min(80);
        let offset = offset.unwrap_or(0);

        let domains = sqlx::query!(
            "SELECT domain FROM domain_blocks WHERE account_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            account_id,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let domain_list: Vec<String> = domains.into_iter().map(|row| row.domain).collect();
        info!(
            "Fetched {} blocked domains for account: {}",
            domain_list.len(),
            account_id
        );
        Ok(domain_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_domain_block_operations() {
        // This is a basic test structure
        // In a real implementation, you would set up a test database
        // and test the actual CRUD operations
        let domain_block = DomainBlock {
            id: 1,
            account_id: 1,
            domain: "example.com".to_string(),
            created_at: None,
        };
        assert_eq!(domain_block.account_id, 1);
        assert_eq!(domain_block.domain, "example.com");
    }
}
