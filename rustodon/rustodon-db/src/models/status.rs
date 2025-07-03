//!
//! Status Model
//!
//! This module provides the status data model and database operations
//! for the Rustodon server, including status creation, retrieval,
//! and timeline management.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)
//!
//! # Dependencies
//!
//! - `sqlx`: Database operations
//! - `serde`: Serialization
//! - `chrono`: DateTime handling
//!
//! # Usage
//!
//! ```rust
//! use rustodon_db::models::status::Status;
//!
//! let status = Status::create(&pool, 1, "Hello world!", "public", false, None, None, None).await?;
//! let timeline = Status::get_public_timeline(&pool, 20, None).await?;
//! ```

use crate::error::DbError;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tracing::{debug, info, warn};

/// Status visibility levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status_visibility", rename_all = "lowercase")]
pub enum StatusVisibility {
    Public,
    Unlisted,
    Private,
    Direct,
}

/// Status type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status_type", rename_all = "lowercase")]
pub enum StatusType {
    Status,
    Reblog,
    Reply,
}

/// Status data model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Status {
    /// Unique identifier
    pub id: i64,
    /// Author user ID
    pub account_id: i64,
    /// Content text
    pub content: String,
    /// Visibility level
    pub visibility: StatusVisibility,
    /// Whether status is sensitive
    pub sensitive: Option<bool>,
    /// Spoiler text
    pub spoiler_text: Option<String>,
    /// In reply to status ID
    pub in_reply_to_id: Option<i64>,
    /// In reply to account ID
    pub in_reply_to_account_id: Option<i64>,
    /// Reblogged status ID
    pub reblog_of_id: Option<i64>,
    /// Status type
    pub status_type: Option<StatusType>,
    /// Language
    pub language: Option<String>,
    /// URL
    pub url: Option<String>,
    /// URI
    pub uri: Option<String>,
    /// When the status was created
    pub created_at: NaiveDateTime,
    /// When the status was last updated
    pub updated_at: NaiveDateTime,
    /// When the status was deleted
    pub deleted_at: Option<NaiveDateTime>,
    /// Whether status is local
    pub local: Option<bool>,
    /// Whether status is federated
    pub federated: Option<bool>,
    /// Favourites count
    pub favourites_count: Option<i64>,
    /// Reblogs count
    pub reblogs_count: Option<i64>,
    /// Replies count
    pub replies_count: Option<i64>,
    /// Media attachments (JSON)
    pub media_attachments: Option<serde_json::Value>,
    /// Mentions (JSON)
    pub mentions: Option<serde_json::Value>,
    /// Tags (JSON)
    pub tags: Option<serde_json::Value>,
    /// Emojis (JSON)
    pub emojis: Option<serde_json::Value>,
    /// Poll (JSON)
    pub poll: Option<serde_json::Value>,
    /// Application (JSON)
    pub application: Option<serde_json::Value>,
}

impl Status {
    /// Create a new status
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - Author user ID
    /// * `content` - Status content
    /// * `visibility` - Visibility level
    /// * `sensitive` - Whether status is sensitive
    /// * `spoiler_text` - Spoiler text (optional)
    /// * `in_reply_to_id` - Reply to status ID (optional)
    /// * `language` - Language code (optional)
    ///
    /// # Returns
    ///
    /// The created status
    ///
    /// # Examples
    ///
    /// ```rust
    /// let status = Status::create(&pool, 1, "Hello world!", StatusVisibility::Public, false, None, None, None).await?;
    /// ```
    pub async fn create(
        pool: &PgPool,
        account_id: i64,
        content: &str,
        visibility: StatusVisibility,
        sensitive: bool,
        spoiler_text: Option<&str>,
        in_reply_to_id: Option<i64>,
        language: Option<&str>,
    ) -> Result<Self, DbError> {
        info!("Creating status for account: {}", account_id);

        // Determine status type
        let status_type = if in_reply_to_id.is_some() {
            StatusType::Reply
        } else {
            StatusType::Status
        };

        // Get reply account ID if replying
        let in_reply_to_account_id = if let Some(reply_id) = in_reply_to_id {
            let reply_status =
                sqlx::query!("SELECT account_id FROM statuses WHERE id = $1", reply_id)
                    .fetch_optional(pool)
                    .await?;

            reply_status.map(|r| r.account_id)
        } else {
            None
        };

        let result = sqlx::query_as!(
            Status,
            r#"
            INSERT INTO statuses (account_id, content, visibility, sensitive, spoiler_text,
                                 in_reply_to_id, in_reply_to_account_id, status_type, language,
                                 local, federated)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, true, true)
            RETURNING id, account_id, content, visibility as "visibility: StatusVisibility",
                      sensitive, spoiler_text, in_reply_to_id, in_reply_to_account_id, reblog_of_id,
                      status_type as "status_type: StatusType", language, url, uri, created_at, updated_at,
                      deleted_at, local, federated, favourites_count, reblogs_count, replies_count,
                      media_attachments, mentions, tags, emojis, poll, application
            "#,
            account_id,
            content,
            visibility as StatusVisibility,
            sensitive,
            spoiler_text,
            in_reply_to_id,
            in_reply_to_account_id,
            status_type as StatusType,
            language,
        )
        .fetch_one(pool)
        .await?;

        // Update user's status count and last status time
        let _ = sqlx::query!(
            r#"
            UPDATE users
            SET statuses_count = statuses_count + 1, last_status_at = NOW()
            WHERE id = $1
            "#,
            account_id,
        )
        .execute(pool)
        .await;

        debug!("Created status with ID: {}", result.id);
        Ok(result)
    }

    /// Get status by ID
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `id` - Status ID
    ///
    /// # Returns
    ///
    /// The status if found
    ///
    /// # Examples
    ///
    /// ```rust
    /// let status = Status::get_by_id(&pool, 1).await?;
    /// ```
    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, DbError> {
        debug!("Looking up status by ID: {}", id);

        let result = sqlx::query_as!(
            Status,
            r#"
            SELECT id, account_id, content, visibility as "visibility: StatusVisibility",
                   sensitive, spoiler_text, in_reply_to_id, in_reply_to_account_id, reblog_of_id,
                   status_type as "status_type: StatusType", language, url, uri, created_at, updated_at,
                   deleted_at, local, federated, favourites_count, reblogs_count, replies_count,
                   media_attachments, mentions, tags, emojis, poll, application
            FROM statuses
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id,
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Get statuses by account
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - Account ID
    /// * `limit` - Maximum number of statuses to return
    /// * `since_id` - Return statuses after this ID (optional)
    ///
    /// # Returns
    ///
    /// Vector of statuses
    ///
    /// # Examples
    ///
    /// ```rust
    /// let statuses = Status::get_by_account(&pool, 1, 20, None).await?;
    /// ```
    pub async fn get_by_account(
        pool: &PgPool,
        account_id: i64,
        limit: i64,
        since_id: Option<i64>,
    ) -> Result<Vec<Self>, DbError> {
        debug!("Getting statuses for account: {}", account_id);

        let result = sqlx::query_as!(
            Status,
            r#"
            SELECT id, account_id, content, visibility as "visibility: StatusVisibility",
                   sensitive, spoiler_text, in_reply_to_id, in_reply_to_account_id, reblog_of_id,
                   status_type as "status_type: StatusType", language, url, uri, created_at, updated_at,
                   deleted_at, local, federated, favourites_count, reblogs_count, replies_count,
                   media_attachments, mentions, tags, emojis, poll, application
            FROM statuses
            WHERE account_id = $1 AND ($2::bigint IS NULL OR id > $2) AND deleted_at IS NULL
            ORDER BY created_at DESC LIMIT $3
            "#,
            account_id,
            since_id,
            limit,
        )
        .fetch_all(pool)
        .await?;

        debug!("Found {} statuses for account: {}", result.len(), account_id);
        Ok(result)
    }

    /// Get public timeline
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `limit` - Maximum number of statuses to return
    /// * `since_id` - Return statuses after this ID (optional)
    ///
    /// # Returns
    ///
    /// Vector of public statuses
    ///
    /// # Examples
    ///
    /// ```rust
    /// let timeline = Status::get_public_timeline(&pool, 20, None).await?;
    /// ```
    pub async fn get_public_timeline(
        pool: &PgPool,
        limit: i64,
        since_id: Option<i64>,
    ) -> Result<Vec<Self>, DbError> {
        debug!("Getting public timeline");

        let result = sqlx::query_as!(
            Status,
            r#"
            SELECT id, account_id, content, visibility as "visibility: StatusVisibility",
                   sensitive, spoiler_text, in_reply_to_id, in_reply_to_account_id, reblog_of_id,
                   status_type as "status_type: StatusType", language, url, uri, created_at, updated_at,
                   deleted_at, local, federated, favourites_count, reblogs_count, replies_count,
                   media_attachments, mentions, tags, emojis, poll, application
            FROM statuses
            WHERE visibility = 'public' AND deleted_at IS NULL AND reblog_of_id IS NULL
                  AND ($1::bigint IS NULL OR id > $1)
            ORDER BY created_at DESC LIMIT $2
            "#,
            since_id,
            limit,
        )
        .fetch_all(pool)
        .await?;

        debug!("Found {} public statuses", result.len());
        Ok(result)
    }

    /// Get home timeline for a user
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - User account ID
    /// * `limit` - Maximum number of statuses to return
    /// * `since_id` - Return statuses after this ID (optional)
    ///
    /// # Returns
    ///
    /// Vector of statuses from followed accounts
    ///
    /// # Examples
    ///
    /// ```rust
    /// let timeline = Status::get_home_timeline(&pool, 1, 20, None).await?;
    /// ```
    pub async fn get_home_timeline(
        pool: &PgPool,
        account_id: i64,
        limit: i64,
        since_id: Option<i64>,
    ) -> Result<Vec<Self>, DbError> {
        debug!("Getting home timeline for account: {}", account_id);

        let result = sqlx::query_as!(
            Status,
            r#"
            SELECT DISTINCT s.id, s.account_id, s.content, s.visibility as "visibility: StatusVisibility",
                   s.sensitive, s.spoiler_text, s.in_reply_to_id, s.in_reply_to_account_id, s.reblog_of_id,
                   s.status_type as "status_type: StatusType", s.language, s.url, s.uri, s.created_at, s.updated_at,
                   s.deleted_at, s.local, s.federated, s.favourites_count, s.reblogs_count, s.replies_count,
                   s.media_attachments, s.mentions, s.tags, s.emojis, s.poll, s.application
            FROM statuses s
            INNER JOIN follows f ON s.account_id = f.target_account_id
            WHERE f.account_id = $1 AND s.deleted_at IS NULL
                  AND ($2::bigint IS NULL OR s.id > $2)
                  AND (s.visibility = 'public' OR s.visibility = 'unlisted' OR s.account_id = $1)
            ORDER BY s.created_at DESC LIMIT $3
            "#,
            account_id,
            since_id,
            limit,
        )
        .fetch_all(pool)
        .await?;

        debug!("Found {} home timeline statuses for account: {}", result.len(), account_id);
        Ok(result)
    }

    /// Delete a status
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    ///
    /// # Returns
    ///
    /// True if status was deleted
    ///
    /// # Examples
    ///
    /// ```rust
    /// let deleted = status.delete(&pool).await?;
    /// ```
    pub async fn delete(&self, pool: &PgPool) -> Result<bool, DbError> {
        info!("Deleting status: {}", self.id);

        let result = sqlx::query!(
            r#"
            UPDATE statuses
            SET deleted_at = NOW()
            WHERE id = $1 AND account_id = $2
            "#,
            self.id,
            self.account_id,
        )
        .execute(pool)
        .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            // Update user's status count
            let _ = sqlx::query!(
                r#"
                UPDATE users
                SET statuses_count = GREATEST(statuses_count - 1, 0)
                WHERE id = $1
                "#,
                self.account_id,
            )
            .execute(pool)
            .await;

            debug!("Deleted status: {}", self.id);
        }

        Ok(deleted)
    }

    /// Update status content
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `content` - New content
    /// * `sensitive` - Whether status is sensitive
    /// * `spoiler_text` - New spoiler text
    ///
    /// # Returns
    ///
    /// The updated status
    ///
    /// # Examples
    ///
    /// ```rust
    /// let status = status.update(&pool, "Updated content", false, None).await?;
    /// ```
    pub async fn update(
        &self,
        pool: &PgPool,
        content: &str,
        sensitive: bool,
        spoiler_text: Option<&str>,
    ) -> Result<Self, DbError> {
        info!("Updating status: {}", self.id);

        let result = sqlx::query_as!(
            Status,
            r#"
            UPDATE statuses
            SET content = $1, sensitive = $2, spoiler_text = $3, updated_at = NOW()
            WHERE id = $4 AND account_id = $5
            RETURNING id, account_id, content, visibility as "visibility: StatusVisibility",
                      sensitive, spoiler_text, in_reply_to_id, in_reply_to_account_id, reblog_of_id,
                      status_type as "status_type: StatusType", language, url, uri, created_at, updated_at,
                      deleted_at, local, federated, favourites_count, reblogs_count, replies_count,
                      media_attachments, mentions, tags, emojis, poll, application
            "#,
            content,
            sensitive,
            spoiler_text,
            self.id,
            self.account_id,
        )
        .fetch_one(pool)
        .await?;

        debug!("Updated status: {}", result.id);
        Ok(result)
    }

    /// Check if status is public
    ///
    /// # Returns
    ///
    /// True if status is public
    pub fn is_public(&self) -> bool {
        matches!(self.visibility, StatusVisibility::Public)
    }

    /// Check if status is a reply
    ///
    /// # Returns
    ///
    /// True if status is a reply
    pub fn is_reply(&self) -> bool {
        self.in_reply_to_id.is_some()
    }

    /// Check if status is a reblog
    ///
    /// # Returns
    ///
    /// True if status is a reblog
    pub fn is_reblog(&self) -> bool {
        self.reblog_of_id.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_status_creation() {
        // This would require a test database setup
        // For now, just test the status methods
        let status = Status {
            id: 1,
            account_id: 1,
            content: "Test status".to_string(),
            visibility: StatusVisibility::Public,
            sensitive: None,
            spoiler_text: None,
            in_reply_to_id: None,
            in_reply_to_account_id: None,
            reblog_of_id: None,
            status_type: None,
            language: Some("en".to_string()),
            url: None,
            uri: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: None,
            local: Some(true),
            federated: Some(true),
            favourites_count: None,
            reblogs_count: None,
            replies_count: None,
            media_attachments: None,
            mentions: None,
            tags: None,
            emojis: None,
            poll: None,
            application: None,
        };

        assert!(status.is_public());
        assert!(!status.is_reply());
        assert!(!status.is_reblog());
    }
}
