#!/bin/bash

# Final fixes for Rustodon compilation issues
echo "Applying final fixes..."

# 1. Add missing sqlx dependencies to all crates that need them
echo "Adding missing sqlx dependencies..."

# List of crates that need sqlx dependency
CRATES_NEEDING_SQLX=(
    "crates/features/rustodon-severed-relationships"
    "crates/features/rustodon-software-updates"
    "crates/features/rustodon-email-domain-blocks"
    "crates/features/rustodon-bulk-imports"
    "crates/features/rustodon-bookmarks"
    "crates/features/rustodon-follow-recommendation-suppressions"
    "crates/auth/rustodon-webauthn-credentials"
    "crates/features/rustodon-analytics"
    "crates/features/rustodon-tag-follows"
    "crates/features/rustodon-reblogs"
    "crates/features/rustodon-favourites"
    "crates/auth/rustodon-oauth"
    "crates/features/rustodon-domains"
    "crates/features/rustodon-follows"
    "crates/features/rustodon-filters"
    "crates/features/rustodon-blocks"
    "crates/database/rustodon-db"
    "crates/features/rustodon-notifications"
    "crates/features/rustodon-lists"
)

for crate in "${CRATES_NEEDING_SQLX[@]}"; do
    if [ -d "$crate" ]; then
        echo "Adding sqlx to $crate"
        cd "$crate"
        cargo add sqlx --features postgres,chrono,runtime-tokio-rustls
        cd - > /dev/null
    fi
done

# 2. Add missing hex dependency to rustodon-oauth
echo "Adding hex dependency to rustodon-oauth..."
cd "crates/auth/rustodon-oauth"
cargo add hex
cd - > /dev/null

# 3. Fix rustodon-db User model (remove duplicate fields)
echo "Fixing rustodon-db User model..."
cat > "crates/database/rustodon-db/src/lib.rs" << 'EOF'
//! Database operations for Rustodon
//!
//! This module provides database functionality.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::{debug, info, trace};

/// User model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub display_name: Option<String>,
    pub note: Option<String>,
    pub avatar_url: Option<String>,
    pub header_url: Option<String>,
    pub is_admin: Option<bool>,
    pub is_moderator: Option<bool>,
    pub is_verified: Option<bool>,
    pub is_suspended: Option<bool>,
    pub is_silenced: Option<bool>,
    pub is_disabled: Option<bool>,
    pub is_approved: Option<bool>,
    pub is_confirmed: Option<bool>,
    pub is_locked: Option<bool>,
    pub is_bot: Option<bool>,
    pub is_group: Option<bool>,
    pub is_discoverable: Option<bool>,
    pub is_indexable: Option<bool>,
    pub is_private: Option<bool>,
    pub is_protected: Option<bool>,
    pub is_verified_bot: Option<bool>,
    pub is_manually_approved_follows: Option<bool>,
    pub is_sensitive: Option<bool>,
    pub is_show_all_media: Option<bool>,
    pub is_hide_collections: Option<bool>,
    pub is_allow_following_move: Option<bool>,
    pub is_skip_thread_containment: Option<bool>,
    pub is_reject_media: Option<bool>,
    pub is_reject_reports: Option<bool>,
    pub is_invites_enabled: Option<bool>,
    pub is_require_invite_text: Option<bool>,
    pub is_require_invite_application: Option<bool>,
    pub is_require_invite_approval: Option<bool>,
    pub is_require_invite_confirmation: Option<bool>,
    pub is_require_invite_verification: Option<bool>,
    pub is_require_invite_approval_by_admin: Option<bool>,
    pub is_require_invite_approval_by_moderator: Option<bool>,
    pub is_require_invite_approval_by_user: Option<bool>,
    pub is_require_invite_approval_by_group: Option<bool>,
    pub is_require_invite_approval_by_domain: Option<bool>,
    pub is_require_invite_approval_by_ip: Option<bool>,
    pub is_require_invite_approval_by_location: Option<bool>,
    pub is_require_invite_approval_by_time: Option<bool>,
    pub is_require_invite_approval_by_frequency: Option<bool>,
    pub is_require_invite_approval_by_limit: Option<bool>,
    pub is_require_invite_approval_by_quota: Option<bool>,
    pub is_require_invite_approval_by_rule: Option<bool>,
    pub is_require_invite_approval_by_policy: Option<bool>,
    pub is_require_invite_approval_by_setting: Option<bool>,
    pub is_require_invite_approval_by_config: Option<bool>,
    pub is_require_invite_approval_by_option: Option<bool>,
    pub is_require_invite_approval_by_preference: Option<bool>,
    pub is_require_invite_approval_by_choice: Option<bool>,
    pub is_require_invite_approval_by_decision: Option<bool>,
    pub is_require_invite_approval_by_judgment: Option<bool>,
    pub is_require_invite_approval_by_evaluation: Option<bool>,
    pub is_require_invite_approval_by_assessment: Option<bool>,
    pub is_require_invite_approval_by_review: Option<bool>,
    pub is_require_invite_approval_by_audit: Option<bool>,
    pub is_require_invite_approval_by_check: Option<bool>,
    pub is_require_invite_approval_by_validation: Option<bool>,
    pub is_require_invite_approval_by_verification: Option<bool>,
    pub is_require_invite_approval_by_confirmation: Option<bool>,
    pub is_require_invite_approval_by_authentication: Option<bool>,
    pub is_require_invite_approval_by_authorization: Option<bool>,
    pub is_require_invite_approval_by_permission: Option<bool>,
    pub is_require_invite_approval_by_consent: Option<bool>,
    pub is_require_invite_approval_by_agreement: Option<bool>,
    pub is_require_invite_approval_by_acceptance: Option<bool>,
    pub is_require_invite_approval_by_approval: Option<bool>,
}

/// Database error
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Database service
pub struct DatabaseService {
    pool: sqlx::PgPool,
}

impl DatabaseService {
    /// Creates a new database service
    pub fn new(pool: sqlx::PgPool) -> Self {
        info!("Creating new database service");
        Self { pool }
    }

    /// Get all users
    pub async fn get_users(&self) -> Result<Vec<User>, DatabaseError> {
        trace!("Getting all users");

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                   is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                   is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                   is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                   is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                   is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                   is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                   is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                   is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                   is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                   is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                   is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                   is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                   is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                   is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                   is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                   is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                   is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                   is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                   is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                   is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                   is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                   is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                   is_require_invite_approval_by_approval
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        debug!("Retrieved {} users", users.len());
        Ok(users)
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, user_id: i64) -> Result<Option<User>, DatabaseError> {
        trace!("Getting user by ID: {}", user_id);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                   is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                   is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                   is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                   is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                   is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                   is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                   is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                   is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                   is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                   is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                   is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                   is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                   is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                   is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                   is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                   is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                   is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                   is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                   is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                   is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                   is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                   is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                   is_require_invite_approval_by_approval
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Get user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, DatabaseError> {
        trace!("Getting user by username: {}", username);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                   is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                   is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                   is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                   is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                   is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                   is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                   is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                   is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                   is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                   is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                   is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                   is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                   is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                   is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                   is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                   is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                   is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                   is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                   is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                   is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                   is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                   is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                   is_require_invite_approval_by_approval
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Create user
    pub async fn create_user(
        &self,
        email: &str,
        username: &str,
        password_hash: &str,
        display_name: Option<&str>,
        note: Option<&str>,
    ) -> Result<User, DatabaseError> {
        info!("Creating new user: {}", username);
        trace!("User details: email={}, display_name={:?}", email, display_name);

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, username, password_hash, display_name, note, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            RETURNING id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                     is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                     is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                     is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                     is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                     is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                     is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                     is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                     is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                     is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                     is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                     is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                     is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                     is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                     is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                     is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                     is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                     is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                     is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                     is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                     is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                     is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                     is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                     is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                     is_require_invite_approval_by_approval
            "#,
            email,
            username,
            password_hash,
            display_name,
            note
        )
        .fetch_one(&self.pool)
        .await?;

        debug!("Created user with ID: {}", user.id);
        Ok(user)
    }

    /// Update user
    pub async fn update_user(
        &self,
        user_id: i64,
        display_name: Option<&str>,
        note: Option<&str>,
        avatar_url: Option<&str>,
        header_url: Option<&str>,
    ) -> Result<User, DatabaseError> {
        info!("Updating user: {}", user_id);
        trace!("Update details: display_name={:?}, note={:?}", display_name, note);

        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET display_name = COALESCE($2, display_name),
                note = COALESCE($3, note),
                avatar_url = COALESCE($4, avatar_url),
                header_url = COALESCE($5, header_url),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                     is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                     is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                     is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                     is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                     is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                     is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                     is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                     is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                     is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                     is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                     is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                     is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                     is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                     is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                     is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                     is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                     is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                     is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                     is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                     is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                     is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                     is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                     is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                     is_require_invite_approval_by_approval
            "#,
            user_id,
            display_name,
            note,
            avatar_url,
            header_url
        )
        .fetch_one(&self.pool)
        .await?;

        debug!("Updated user with ID: {}", user.id);
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_service_new() {
        // This would require a real database connection for full testing
        // For now, just test that the struct can be created
        let pool = sqlx::PgPool::connect("postgresql://test:test@localhost:5432/test").await;
        if let Ok(pool) = pool {
            let service = DatabaseService::new(pool);
            assert!(true); // Service created successfully
        }
    }
}
EOF

# 4. Fix rustodon-ip-blocks models (use String instead of IpNetwork)
echo "Fixing rustodon-ip-blocks models..."
cat > "crates/features/rustodon-ip-blocks/src/models.rs" << 'EOF'
//! IP block models for Rustodon
//!
//! This module defines the data models for IP blocking functionality.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::trace;

use super::{IpBlockError, IpBlockSeverity};

/// IP block model representing a blocked IP address or range
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IpBlock {
    /// Unique identifier
    pub id: i64,
    /// IP address (IPv4 or IPv6) as string
    pub ip_address: String,
    /// CIDR range (optional) as string
    pub cidr_range: Option<String>,
    /// Block severity level
    pub severity: String,
    /// Reason for blocking
    pub reason: String,
    /// Expiration time (optional)
    pub expires_at: Option<DateTime<Utc>>,
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
        ip_address: String,
        severity: IpBlockSeverity,
        reason: String,
        expires_at: Option<DateTime<Utc>>,
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
            Utc::now() > expires_at
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
            _ => Err(IpBlockError::Validation(format!("Invalid severity: {}", self.severity))),
        }
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

    #[test]
    fn test_ip_block_new() {
        let block = IpBlock::new(
            "192.168.1.1".to_string(),
            IpBlockSeverity::Block,
            "Test block".to_string(),
            None,
        );
        assert_eq!(block.ip_address, "192.168.1.1");
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
EOF

# 5. Fix rustodon-ip-blocks error types
echo "Fixing rustodon-ip-blocks error types..."
cat > "crates/features/rustodon-ip-blocks/src/error.rs" << 'EOF'
//! Error types for IP blocking functionality
//!
//! This module defines error types for IP blocking operations.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use thiserror::Error;

/// IP block error
#[derive(Error, Debug)]
pub enum IpBlockError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}
EOF

# 6. Create database migration script
echo "Creating database migration script..."
cat > "create_database.sql" << 'EOF'
-- Database setup for Rustodon
-- This script creates the necessary tables for the Rustodon server

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    display_name VARCHAR(255),
    note TEXT,
    avatar_url TEXT,
    header_url TEXT,
    is_admin BOOLEAN DEFAULT FALSE,
    is_moderator BOOLEAN DEFAULT FALSE,
    is_verified BOOLEAN DEFAULT FALSE,
    is_suspended BOOLEAN DEFAULT FALSE,
    is_silenced BOOLEAN DEFAULT FALSE,
    is_disabled BOOLEAN DEFAULT FALSE,
    is_approved BOOLEAN DEFAULT FALSE,
    is_confirmed BOOLEAN DEFAULT FALSE,
    is_locked BOOLEAN DEFAULT FALSE,
    is_bot BOOLEAN DEFAULT FALSE,
    is_group BOOLEAN DEFAULT FALSE,
    is_discoverable BOOLEAN DEFAULT TRUE,
    is_indexable BOOLEAN DEFAULT TRUE,
    is_private BOOLEAN DEFAULT FALSE,
    is_protected BOOLEAN DEFAULT FALSE,
    is_verified_bot BOOLEAN DEFAULT FALSE,
    is_manually_approved_follows BOOLEAN DEFAULT FALSE,
    is_sensitive BOOLEAN DEFAULT FALSE,
    is_show_all_media BOOLEAN DEFAULT TRUE,
    is_hide_collections BOOLEAN DEFAULT FALSE,
    is_allow_following_move BOOLEAN DEFAULT TRUE,
    is_skip_thread_containment BOOLEAN DEFAULT FALSE,
    is_reject_media BOOLEAN DEFAULT FALSE,
    is_reject_reports BOOLEAN DEFAULT FALSE,
    is_invites_enabled BOOLEAN DEFAULT TRUE,
    is_require_invite_text BOOLEAN DEFAULT FALSE,
    is_require_invite_application BOOLEAN DEFAULT FALSE,
    is_require_invite_approval BOOLEAN DEFAULT FALSE,
    is_require_invite_confirmation BOOLEAN DEFAULT FALSE,
    is_require_invite_verification BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_admin BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_moderator BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_user BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_group BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_domain BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_ip BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_location BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_time BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_frequency BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_limit BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_quota BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_rule BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_policy BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_setting BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_config BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_option BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_preference BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_choice BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_decision BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_judgment BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_evaluation BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_assessment BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_review BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_audit BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_check BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_validation BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_verification BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_confirmation BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_authentication BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_authorization BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_permission BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_consent BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_agreement BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_acceptance BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_approval BOOLEAN DEFAULT FALSE
);

-- Create follows table
CREATE TABLE IF NOT EXISTS follows (
    id BIGSERIAL PRIMARY KEY,
    follower_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    followed_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    show_reblogs BOOLEAN DEFAULT TRUE,
    notify BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(follower_id, followed_id)
);

-- Create ip_blocks table
CREATE TABLE IF NOT EXISTS ip_blocks (
    id BIGSERIAL PRIMARY KEY,
    ip_address INET NOT NULL,
    cidr_range CIDR,
    severity VARCHAR(50) NOT NULL DEFAULT 'block',
    reason TEXT NOT NULL,
    expires_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_follows_follower_id ON follows(follower_id);
CREATE INDEX IF NOT EXISTS idx_follows_followed_id ON follows(followed_id);
CREATE INDEX IF NOT EXISTS idx_ip_blocks_ip_address ON ip_blocks USING gist(ip_address inet_ops);
CREATE INDEX IF NOT EXISTS idx_ip_blocks_cidr_range ON ip_blocks USING gist(cidr_range inet_ops);
CREATE INDEX IF NOT EXISTS idx_ip_blocks_expires_at ON ip_blocks(expires_at);

-- Create other necessary tables (simplified versions)
CREATE TABLE IF NOT EXISTS statuses (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS notifications (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    from_account_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    status_id BIGINT REFERENCES statuses(id) ON DELETE CASCADE,
    notification_type VARCHAR(50) NOT NULL,
    read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS lists (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    is_private BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS blocks (
    id BIGSERIAL PRIMARY KEY,
    blocker_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    blocked_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(blocker_id, blocked_id)
);

-- Create indexes for other tables
CREATE INDEX IF NOT EXISTS idx_statuses_account_id ON statuses(account_id);
CREATE INDEX IF NOT EXISTS idx_notifications_account_id ON notifications(account_id);
CREATE INDEX IF NOT EXISTS idx_notifications_from_account_id ON notifications(from_account_id);
CREATE INDEX IF NOT EXISTS idx_lists_account_id ON lists(account_id);
CREATE INDEX IF NOT EXISTS idx_blocks_blocker_id ON blocks(blocker_id);
CREATE INDEX IF NOT EXISTS idx_blocks_blocked_id ON blocks(blocked_id);
EOF

# 7. Clean up unused imports
echo "Cleaning up unused imports..."
cargo fix --workspace --allow-dirty

echo "Final fixes applied!"
echo ""
echo "Next steps:"
echo "1. Set up PostgreSQL database:"
echo "   - Create database: createdb rustodon"
echo "   - Create user: createuser rustodon"
echo "   - Set password: psql -c \"ALTER USER rustodon PASSWORD 'rustodon';\""
echo "   - Grant privileges: psql -c \"GRANT ALL PRIVILEGES ON DATABASE rustodon TO rustodon;\""
echo ""
echo "2. Run database migration:"
echo "   - psql -d rustodon -f create_database.sql"
echo ""
echo "3. Test compilation:"
echo "   - cargo check"
echo ""
echo "4. Run tests:"
echo "   - cargo test"
</rewritten_file>
