//! IP block service for Rustodon
//!
//! This module provides IP blocking functionality.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::{Duration, Utc};
use sqlx::PgPool;
use tracing::{debug, info, trace};

use super::{models::IpBlock, CreateIpBlockRequest, IpBlockError, IpBlockQuery};

/// IP block service
pub struct IpBlockService {
    pool: PgPool,
}

impl IpBlockService {
    /// Creates a new IP block service
    pub fn new(pool: PgPool) -> Self {
        info!("Creating new IP block service");
        Self { pool }
    }

    /// Create an IP block
    ///
    /// # Arguments
    ///
    /// * `request` - Create request
    ///
    /// # Returns
    ///
    /// Result containing the created IP block or error
    pub async fn create_ip_block(
        &self,
        request: CreateIpBlockRequest,
    ) -> Result<IpBlock, IpBlockError> {
        info!("Creating IP block for: {}", request.ip_address);
        trace!("Create request: {:?}", request);

        // IP address and CIDR range are already validated as IpNetwork types

        // Calculate expiration time
        let expires_at = request
            .duration
            .map(|duration| Utc::now().naive_utc() + Duration::seconds(duration as i64));

        // Create the block
        let reason = request
            .reason
            .unwrap_or_else(|| "No reason provided".to_string());
        let result = sqlx::query_as_unchecked!(
            IpBlock,
            r#"
            INSERT INTO ip_blocks (ip_address, cidr_range, severity, reason, expires_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, ip_address, cidr_range, severity, reason, expires_at, created_at, updated_at
            "#,
            request.ip_address,
            request.cidr_range,
            request.severity.to_string(),
            reason,
            expires_at,
        )
        .fetch_one(&self.pool)
        .await?;

        debug!("Created IP block with ID: {}", result.id);
        Ok(result)
    }

    /// Get IP block by ID
    ///
    /// # Arguments
    ///
    /// * `id` - Block ID
    ///
    /// # Returns
    ///
    /// Result containing the IP block or error
    pub async fn get_ip_block(&self, id: i64) -> Result<Option<IpBlock>, IpBlockError> {
        trace!("Getting IP block by ID: {}", id);

        let result = sqlx::query_as_unchecked!(
            IpBlock,
            r#"
            SELECT id, ip_address, cidr_range, severity, reason, expires_at, created_at, updated_at
            FROM ip_blocks
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    /// Delete IP block
    ///
    /// # Arguments
    ///
    /// * `id` - Block ID
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn delete_ip_block(&self, id: i64) -> Result<bool, IpBlockError> {
        info!("Deleting IP block with ID: {}", id);

        let result = sqlx::query!(
            r#"
            DELETE FROM ip_blocks
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        let deleted = result.rows_affected() > 0;
        debug!("Deleted IP block with ID: {} (deleted: {})", id, deleted);
        Ok(deleted)
    }

    /// List IP blocks
    ///
    /// # Arguments
    ///
    /// * `query` - Query parameters
    ///
    /// # Returns
    ///
    /// Result containing list of IP blocks or error
    pub async fn list_ip_blocks(&self, query: IpBlockQuery) -> Result<Vec<IpBlock>, IpBlockError> {
        trace!("Listing IP blocks with query: {:?}", query);

        let mut sql = String::from(
            r#"
            SELECT id, ip_address, cidr_range, severity, reason, expires_at, created_at, updated_at
            FROM ip_blocks
            WHERE 1=1
            "#,
        );
        let mut params: Vec<String> = Vec::new();

        if let Some(ref ip_address) = query.ip_address {
            sql.push_str(&format!(" AND ip_address = ${}", params.len() + 1));
            params.push(ip_address.clone());
        }

        if let Some(ref severity) = query.severity {
            sql.push_str(&format!(" AND severity = ${}", params.len() + 1));
            params.push(severity.to_string());
        }

        sql.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = query.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(page) = query.page {
            let offset = (page - 1) * query.limit.unwrap_or(20);
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut query = sqlx::query_as::<_, IpBlock>(&sql);

        for param in params {
            query = query.bind(param);
        }

        let blocks = query.fetch_all(&self.pool).await?;

        debug!("Retrieved {} IP blocks", blocks.len());
        Ok(blocks)
    }

    /// Cleanup expired blocks
    ///
    /// # Returns
    ///
    /// Result containing number of cleaned blocks or error
    pub async fn cleanup_expired_blocks(&self) -> Result<u64, IpBlockError> {
        info!("Cleaning up expired IP blocks");

        let result = sqlx::query!(
            r#"
            DELETE FROM ip_blocks
            WHERE expires_at IS NOT NULL AND expires_at < NOW()
            "#
        )
        .execute(&self.pool)
        .await?;

        let cleaned = result.rows_affected();
        debug!("Cleaned up {} expired IP blocks", cleaned);
        Ok(cleaned)
    }

    /// Validate IP address
    ///
    /// # Arguments
    ///
    /// * `ip_address` - IP address to validate
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    #[allow(dead_code)]
    fn validate_ip_address(&self, ip_address: &str) -> Result<(), IpBlockError> {
        if ip_address.parse::<std::net::IpAddr>().is_ok() {
            Ok(())
        } else {
            Err(IpBlockError::Validation(format!(
                "Invalid IP address: {}",
                ip_address
            )))
        }
    }

    /// Validate CIDR range
    ///
    /// # Arguments
    ///
    /// * `cidr_range` - CIDR range to validate
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    #[allow(dead_code)]
    fn validate_cidr_range(&self, cidr_range: &str) -> Result<(), IpBlockError> {
        if cidr_range.parse::<ipnetwork::IpNetwork>().is_ok() {
            Ok(())
        } else {
            Err(IpBlockError::Validation(format!(
                "Invalid CIDR range: {}",
                cidr_range
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ip_block_service_new() {
        // This would require a real database connection for full testing
        // For now, just test that the struct can be created
        let pool = PgPool::connect("postgresql://test:test@localhost:5432/test").await;
        if let Ok(pool) = pool {
            let _service = IpBlockService::new(pool);
            // Service created successfully
        }
    }
}
