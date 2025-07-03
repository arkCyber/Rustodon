//! IP block service for Rustodon
//!
//! This module provides the service layer for IP blocking functionality.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use ipnetwork::IpNetwork;
use sqlx::PgPool;
use std::str::FromStr;
use tracing::{debug, error, info, trace, warn};

use super::{
    CreateIpBlockRequest, IpBlock, IpBlockError, IpBlockQuery, IpBlockSeverity,
    UpdateIpBlockRequest,
};

/// Service for managing IP blocks
#[derive(Debug, Clone)]
pub struct IpBlockService {
    pool: PgPool,
}

impl IpBlockService {
    /// Creates a new IP block service
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    ///
    /// # Returns
    ///
    /// A new IpBlockService instance
    pub fn new(pool: PgPool) -> Self {
        trace!("Creating new IP block service");
        Self { pool }
    }

    /// Create a new IP block
    ///
    /// # Arguments
    ///
    /// * `request` - IP block creation request
    ///
    /// # Returns
    ///
    /// The created IP block
    pub async fn create_block(
        &self,
        request: CreateIpBlockRequest,
    ) -> Result<IpBlock, IpBlockError> {
        info!("Creating IP block for: {}", request.ip_address);
        trace!("IP block request: {:?}", request);

        // Validate IP address
        self.validate_ip_address(&request.ip_address)?;

        // Validate CIDR range if present
        if let Some(ref cidr) = request.cidr_range {
            self.validate_cidr_range(cidr)?;
        }

        // Calculate expiration time
        let expires_at = request
            .duration
            .map(|duration| Utc::now() + Duration::seconds(duration as i64));

        // Create the block
        let block = IpBlock::new(
            request.ip_address,
            request.severity,
            request.reason,
            expires_at,
        );

        // Insert into database
        let result = sqlx::query_as!(
            IpBlock,
            r#"
            INSERT INTO ip_blocks (ip_address, cidr_range, severity, reason, expires_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, ip_address, cidr_range, severity, reason, expires_at, created_at, updated_at
            "#,
            block.ip_address,
            block.cidr_range,
            block.severity,
            block.reason,
            block.expires_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to create IP block: {}", e);
            IpBlockError::Database(e)
        })?;

        debug!("Created IP block with ID: {}", result.id);
        Ok(result)
    }

    /// Get an IP block by ID
    ///
    /// # Arguments
    ///
    /// * `id` - IP block ID
    ///
    /// # Returns
    ///
    /// The IP block if found
    pub async fn get_block(&self, id: i64) -> Result<Option<IpBlock>, IpBlockError> {
        trace!("Getting IP block with ID: {}", id);

        let result = sqlx::query_as!(
            IpBlock,
            r#"
            SELECT id, ip_address, cidr_range, severity, reason, expires_at, created_at, updated_at
            FROM ip_blocks
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to get IP block: {}", e);
            IpBlockError::Database(e)
        })?;

        Ok(result)
    }

    /// Get IP blocks matching an IP address
    ///
    /// # Arguments
    ///
    /// * `ip_address` - IP address to check
    ///
    /// # Returns
    ///
    /// List of matching IP blocks
    pub async fn get_blocks_for_ip(&self, ip_address: &str) -> Result<Vec<IpBlock>, IpBlockError> {
        trace!("Getting IP blocks for: {}", ip_address);

        let blocks = sqlx::query_as!(
            IpBlock,
            r#"
            SELECT id, ip_address, cidr_range, severity, reason, expires_at, created_at, updated_at
            FROM ip_blocks
            WHERE ip_address = $1 OR (cidr_range IS NOT NULL AND $1::inet << cidr_range::inet)
            AND (expires_at IS NULL OR expires_at > NOW())
            ORDER BY created_at DESC
            "#,
            ip_address
                .parse::<IpNetwork>()
                .unwrap_or_else(|_| { "127.0.0.1".parse::<IpNetwork>().unwrap() }),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to get IP blocks for IP: {}", e);
            IpBlockError::Database(e)
        })?;

        debug!("Found {} IP blocks for {}", blocks.len(), ip_address);
        Ok(blocks)
    }

    /// Check if an IP address is blocked
    ///
    /// # Arguments
    ///
    /// * `ip_address` - IP address to check
    ///
    /// # Returns
    ///
    /// True if the IP is blocked
    pub async fn is_ip_blocked(&self, ip_address: &str) -> Result<bool, IpBlockError> {
        trace!("Checking if IP is blocked: {}", ip_address);

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM ip_blocks
            WHERE (ip_address = $1 OR (cidr_range IS NOT NULL AND $1::inet << cidr_range::inet))
            AND (expires_at IS NULL OR expires_at > NOW())
            "#,
            ip_address
                .parse::<IpNetwork>()
                .unwrap_or_else(|_| { "127.0.0.1".parse::<IpNetwork>().unwrap() }),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to check if IP is blocked: {}", e);
            IpBlockError::Database(e)
        })?;

        Ok(count.count.unwrap_or(0) > 0)
    }

    /// Update an IP block
    ///
    /// # Arguments
    ///
    /// * `id` - IP block ID
    /// * `request` - Update request
    ///
    /// # Returns
    ///
    /// The updated IP block
    pub async fn update_block(
        &self,
        id: i64,
        request: UpdateIpBlockRequest,
    ) -> Result<Option<IpBlock>, IpBlockError> {
        info!("Updating IP block with ID: {}", id);
        trace!("Update request: {:?}", request);

        // Check if block exists
        let existing = self.get_block(id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        // Build update query
        let mut query = String::from("UPDATE ip_blocks SET updated_at = NOW()");
        let mut params: Vec<String> = vec![];
        let mut param_count = 1;

        if let Some(severity) = request.severity {
            query.push_str(&format!(", severity = ${}", param_count));
            params.push(severity.to_string());
            param_count += 1;
        }

        if let Some(reason) = request.reason {
            query.push_str(&format!(", reason = ${}", param_count));
            params.push(reason);
            param_count += 1;
        }

        if let Some(duration) = request.duration {
            let expires_at = Utc::now() + Duration::seconds(duration as i64);
            query.push_str(&format!(", expires_at = ${}", param_count));
            params.push(expires_at.to_rfc3339());
            param_count += 1;
        }

        query.push_str(&format!(" WHERE id = ${}", param_count));
        params.push(id.to_string());

        query.push_str(" RETURNING id, ip_address, cidr_range, severity, reason, expires_at, created_at, updated_at");

        // Execute update
        let result = sqlx::query_as::<_, IpBlock>(&query)
            .bind(&params)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to update IP block: {}", e);
                IpBlockError::Database(e)
            })?;

        debug!("Updated IP block with ID: {}", id);
        Ok(result)
    }

    /// Delete an IP block
    ///
    /// # Arguments
    ///
    /// * `id` - IP block ID
    ///
    /// # Returns
    ///
    /// True if the block was deleted
    pub async fn delete_block(&self, id: i64) -> Result<bool, IpBlockError> {
        info!("Deleting IP block with ID: {}", id);

        let result = sqlx::query!("DELETE FROM ip_blocks WHERE id = $1", id,)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to delete IP block: {}", e);
                IpBlockError::Database(e)
            })?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            debug!("Deleted IP block with ID: {}", id);
        } else {
            warn!("IP block with ID {} not found for deletion", id);
        }

        Ok(deleted)
    }

    /// List IP blocks with pagination
    ///
    /// # Arguments
    ///
    /// * `query` - Query parameters
    ///
    /// # Returns
    ///
    /// List of IP blocks
    pub async fn list_blocks(&self, query: IpBlockQuery) -> Result<Vec<IpBlock>, IpBlockError> {
        trace!("Listing IP blocks with query: {:?}", query);

        let mut sql = String::from(
            "SELECT id, ip_address, cidr_range, severity, reason, expires_at, created_at, updated_at FROM ip_blocks WHERE 1=1",
        );
        let mut params: Vec<String> = vec![];
        let mut param_count = 1;

        if let Some(ip_address) = query.ip_address {
            sql.push_str(&format!(" AND ip_address = ${}", param_count));
            params.push(ip_address);
            param_count += 1;
        }

        if let Some(severity) = query.severity {
            sql.push_str(&format!(" AND severity = ${}", param_count));
            params.push(severity.to_string());
            param_count += 1;
        }

        if !query.include_expired {
            sql.push_str(" AND (expires_at IS NULL OR expires_at > NOW())");
        }

        sql.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = query.limit {
            sql.push_str(&format!(" LIMIT ${}", param_count));
            params.push(limit.to_string());
            param_count += 1;

            if let Some(page) = query.page {
                let offset = (page - 1) * limit;
                sql.push_str(&format!(" OFFSET ${}", param_count));
                params.push(offset.to_string());
            }
        }

        let blocks = sqlx::query_as::<_, IpBlock>(&sql)
            .bind(&params)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to list IP blocks: {}", e);
                IpBlockError::Database(e)
            })?;

        debug!("Found {} IP blocks", blocks.len());
        Ok(blocks)
    }

    /// Clean up expired IP blocks
    ///
    /// # Returns
    ///
    /// Number of blocks cleaned up
    pub async fn cleanup_expired(&self) -> Result<u64, IpBlockError> {
        info!("Cleaning up expired IP blocks");

        let result = sqlx::query!(
            "DELETE FROM ip_blocks WHERE expires_at IS NOT NULL AND expires_at <= NOW()",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to cleanup expired IP blocks: {}", e);
            IpBlockError::Database(e)
        })?;

        let deleted = result.rows_affected();
        info!("Cleaned up {} expired IP blocks", deleted);
        Ok(deleted)
    }

    /// Validate IP address format
    ///
    /// # Arguments
    ///
    /// * `ip_address` - IP address to validate
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    fn validate_ip_address(&self, ip_address: &str) -> Result<(), IpBlockError> {
        if ip_address.parse::<std::net::IpAddr>().is_err() {
            return Err(IpBlockError::InvalidIpAddress(ip_address.to_string()));
        }
        Ok(())
    }

    /// Validate CIDR range format
    ///
    /// # Arguments
    ///
    /// * `cidr_range` - CIDR range to validate
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    fn validate_cidr_range(&self, cidr_range: &str) -> Result<(), IpBlockError> {
        if cidr_range.parse::<IpNetwork>().is_err() {
            return Err(IpBlockError::InvalidCidrRange(cidr_range.to_string()));
        }
        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_ip_block_service_new() {
        let pool = PgPool::connect("postgres://test:test@localhost/test")
            .await
            .unwrap();
        let service = IpBlockService::new(pool);
        assert!(service.pool.acquire().await.is_ok());
    }

    #[tokio::test]
    async fn test_ip_block_severity_display() {
        assert_eq!(IpBlockSeverity::Noop.to_string(), "noop");
        assert_eq!(IpBlockSeverity::Suspend.to_string(), "suspend");
        assert_eq!(IpBlockSeverity::Silence.to_string(), "silence");
        assert_eq!(IpBlockSeverity::Block.to_string(), "block");
    }
}
