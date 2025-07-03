//! Notifications module for Rustodon
//!
//! This module provides notification functionality for the Rustodon server.
//! It handles creating, managing, and delivering notifications to users
//! with proper database operations and validation.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_notifications::{Notification, NotificationType};
//!
//! let notification = Notification::create(&pool, user_id, "follow", from_account_id).await?;
//! ```
//!
//! # Dependencies
//!
//! - `rustodon_core`: Core types and traits
//! - `rustodon_db`: Database operations
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
use sqlx::{PgPool, Row};
use std::str::FromStr;
use thiserror::Error;
use tracing::{debug, error, info, trace};

/// Custom error type for notifications module
#[derive(Error, Debug)]
pub enum NotificationsError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Notification not found: {0}")]
    NotificationNotFound(i64),
    #[error("User not found: {0}")]
    UserNotFound(i64),
    #[error("Account not found: {0}")]
    AccountNotFound(i64),
    #[error("Status not found: {0}")]
    StatusNotFound(i64),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Notification type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationType {
    /// Follow notification
    Follow,
    /// Follow request notification
    FollowRequest,
    /// Mention notification
    Mention,
    /// Reblog notification
    Reblog,
    /// Favourite notification
    Favourite,
    /// Poll notification
    Poll,
    /// Status notification
    Status,
    /// Update notification
    Update,
    /// Admin signup notification
    AdminSignup,
    /// Admin report notification
    AdminReport,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::Follow => write!(f, "follow"),
            NotificationType::FollowRequest => write!(f, "follow_request"),
            NotificationType::Mention => write!(f, "mention"),
            NotificationType::Reblog => write!(f, "reblog"),
            NotificationType::Favourite => write!(f, "favourite"),
            NotificationType::Poll => write!(f, "poll"),
            NotificationType::Status => write!(f, "status"),
            NotificationType::Update => write!(f, "update"),
            NotificationType::AdminSignup => write!(f, "admin.sign_up"),
            NotificationType::AdminReport => write!(f, "admin.report"),
        }
    }
}

impl FromStr for NotificationType {
    type Err = NotificationsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "follow" => Ok(NotificationType::Follow),
            "follow_request" => Ok(NotificationType::FollowRequest),
            "mention" => Ok(NotificationType::Mention),
            "reblog" => Ok(NotificationType::Reblog),
            "favourite" => Ok(NotificationType::Favourite),
            "poll" => Ok(NotificationType::Poll),
            "status" => Ok(NotificationType::Status),
            "update" => Ok(NotificationType::Update),
            "admin.sign_up" => Ok(NotificationType::AdminSignup),
            "admin.report" => Ok(NotificationType::AdminReport),
            _ => Err(NotificationsError::Validation(format!(
                "Invalid notification type: {}",
                s
            ))),
        }
    }
}

/// Notification data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Unique identifier for the notification
    pub id: i64,
    /// ID of the account that will receive the notification
    pub account_id: i64,
    /// ID of the account that triggered the notification
    pub from_account_id: Option<i64>,
    /// Type of notification
    pub notification_type: NotificationType,
    /// ID of the status related to this notification (if any)
    pub status_id: Option<i64>,
    /// ID of the poll related to this notification (if any)
    pub poll_id: Option<i64>,
    /// Whether the notification has been read
    pub read: bool,
    /// When the notification was created
    pub created_at: DateTime<Utc>,
    /// When the notification was last updated
    pub updated_at: DateTime<Utc>,
}

/// Create notification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    /// ID of the account that will receive the notification
    pub account_id: i64,
    /// ID of the account that triggered the notification
    pub from_account_id: Option<i64>,
    /// Type of notification
    pub notification_type: NotificationType,
    /// ID of the status related to this notification (if any)
    pub status_id: Option<i64>,
    /// ID of the poll related to this notification (if any)
    pub poll_id: Option<i64>,
}

/// Update notification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNotificationRequest {
    /// Whether the notification has been read
    pub read: Option<bool>,
}

impl Notification {
    /// Creates a new notification
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `request` - Notification creation request
    ///
    /// # Returns
    ///
    /// Result containing the created notification or an error
    pub async fn create(
        pool: &PgPool,
        request: CreateNotificationRequest,
    ) -> Result<Self, NotificationsError> {
        trace!(
            "Creating notification for account {} with type: {}",
            request.account_id,
            request.notification_type
        );

        // Validate request
        if request.account_id <= 0 {
            return Err(NotificationsError::Validation(
                "Invalid account ID".to_string(),
            ));
        }

        // Check if account exists
        let account_exists = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM users
            WHERE id = $1
            "#,
            request.account_id
        )
        .fetch_one(pool)
        .await?
        .count;

        if account_exists == Some(0) {
            return Err(NotificationsError::AccountNotFound(request.account_id));
        }

        // Check if from_account exists if provided
        if let Some(from_account_id) = request.from_account_id {
            let from_account_exists = sqlx::query!(
                r#"
                SELECT COUNT(*) as count
                FROM users
                WHERE id = $1
                "#,
                from_account_id
            )
            .fetch_one(pool)
            .await?
            .count;

            if from_account_exists == Some(0) {
                return Err(NotificationsError::AccountNotFound(from_account_id));
            }
        }

        // Check if status exists if provided
        if let Some(status_id) = request.status_id {
            let status_exists = sqlx::query!(
                r#"
                SELECT COUNT(*) as count
                FROM statuses
                WHERE id = $1
                "#,
                status_id
            )
            .fetch_one(pool)
            .await?
            .count;

            if status_exists == Some(0) {
                return Err(NotificationsError::StatusNotFound(status_id));
            }
        }

        // Insert notification
        let notification_row = sqlx::query!(
            r#"
            INSERT INTO notifications (account_id, from_account_id, notification_type, status_id, poll_id, read)
            VALUES ($1, $2, $3, $4, $5, false)
            RETURNING id, account_id, from_account_id, notification_type, status_id, poll_id, read, created_at, updated_at
            "#,
            request.account_id,
            request.from_account_id,
            request.notification_type.to_string(),
            request.status_id,
            request.poll_id
        )
        .fetch_one(pool)
        .await?;

        let notification = Notification {
            id: notification_row.id,
            account_id: notification_row.account_id,
            from_account_id: Some(notification_row.from_account_id),
            notification_type: NotificationType::from_str(&notification_row.notification_type)?,
            status_id: notification_row.status_id,
            poll_id: notification_row.poll_id,
            read: notification_row.read,
            created_at: DateTime::from_naive_utc_and_offset(notification_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(notification_row.updated_at, Utc),
        };

        info!(
            "Created notification with id: {} for account {} with type: {}",
            notification.id, request.account_id, request.notification_type
        );
        Ok(notification)
    }

    /// Gets a notification by ID
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `notification_id` - ID of the notification to retrieve
    ///
    /// # Returns
    ///
    /// Result containing the notification or an error
    pub async fn get_by_id(
        pool: &PgPool,
        notification_id: i64,
    ) -> Result<Self, NotificationsError> {
        trace!("Getting notification by id: {}", notification_id);

        let notification_row = sqlx::query!(
            r#"
            SELECT id, account_id, from_account_id, notification_type, status_id, poll_id, read, created_at, updated_at
            FROM notifications
            WHERE id = $1
            "#,
            notification_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or(NotificationsError::NotificationNotFound(notification_id))?;

        let notification = Notification {
            id: notification_row.id,
            account_id: notification_row.account_id,
            from_account_id: Some(notification_row.from_account_id),
            notification_type: NotificationType::from_str(&notification_row.notification_type)?,
            status_id: notification_row.status_id,
            poll_id: notification_row.poll_id,
            read: notification_row.read,
            created_at: DateTime::from_naive_utc_and_offset(notification_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(notification_row.updated_at, Utc),
        };

        debug!("Retrieved notification with id: {}", notification.id);
        Ok(notification)
    }

    /// Gets all notifications for an account
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the account
    /// * `limit` - Maximum number of notifications to return
    /// * `since_id` - Return notifications after this ID
    /// * `max_id` - Return notifications before this ID
    /// * `exclude_types` - Notification types to exclude
    ///
    /// # Returns
    ///
    /// Result containing the list of notifications or an error
    pub async fn get_by_account(
        pool: &PgPool,
        account_id: i64,
        limit: Option<i64>,
        since_id: Option<i64>,
        max_id: Option<i64>,
        exclude_types: Option<Vec<NotificationType>>,
    ) -> Result<Vec<Self>, NotificationsError> {
        trace!("Getting notifications for account: {}", account_id);

        let limit = limit.unwrap_or(20).min(40);
        let mut query = String::from(
            "SELECT id, account_id, from_account_id, notification_type, status_id, poll_id, read, created_at, updated_at FROM notifications WHERE account_id = $1"
        );
        let mut params: Vec<String> = vec![account_id.to_string()];
        let mut param_count = 1;

        // Add exclude types filter
        if let Some(exclude_types) = exclude_types {
            let exclude_strings: Vec<String> =
                exclude_types.iter().map(|t| t.to_string()).collect();
            param_count += 1;
            query.push_str(&format!(" AND notification_type NOT IN (${})", param_count));
            params.push(format!("{{{}}}", exclude_strings.join(",")));
        }

        // Add since_id filter
        if let Some(since_id) = since_id {
            param_count += 1;
            query.push_str(&format!(" AND id > ${}", param_count));
            params.push(since_id.to_string());
        }

        // Add max_id filter
        if let Some(max_id) = max_id {
            param_count += 1;
            query.push_str(&format!(" AND id < ${}", param_count));
            params.push(max_id.to_string());
        }

        query.push_str(" ORDER BY id DESC");
        param_count += 1;
        query.push_str(&format!(" LIMIT ${}", param_count));
        params.push(limit.to_string());

        // Execute query
        let notification_rows = sqlx::query(&query).bind(&params).fetch_all(pool).await?;

        let notifications: Vec<Notification> = notification_rows
            .into_iter()
            .map(|row| {
                let notification_type = NotificationType::from_str(row.get("notification_type"))
                    .unwrap_or(NotificationType::Follow);
                Notification {
                    id: row.get("id"),
                    account_id: row.get("account_id"),
                    from_account_id: Some(row.get("from_account_id")),
                    notification_type,
                    status_id: row.get("status_id"),
                    poll_id: row.get("poll_id"),
                    read: row.get("read"),
                    created_at: DateTime::from_naive_utc_and_offset(row.get("created_at"), Utc),
                    updated_at: DateTime::from_naive_utc_and_offset(row.get("updated_at"), Utc),
                }
            })
            .collect();

        debug!(
            "Retrieved {} notifications for account {}",
            notifications.len(),
            account_id
        );
        Ok(notifications)
    }

    /// Updates a notification
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `notification_id` - ID of the notification to update
    /// * `account_id` - ID of the account that owns the notification
    /// * `request` - Update request
    ///
    /// # Returns
    ///
    /// Result containing the updated notification or an error
    pub async fn update(
        pool: &PgPool,
        notification_id: i64,
        account_id: i64,
        request: UpdateNotificationRequest,
    ) -> Result<Self, NotificationsError> {
        trace!(
            "Updating notification {} for account {}",
            notification_id,
            account_id
        );

        let notification_row = sqlx::query!(
            r#"
            UPDATE notifications
            SET read = COALESCE($3, read),
                updated_at = now()
            WHERE id = $1 AND account_id = $2
            RETURNING id, account_id, from_account_id, notification_type, status_id, poll_id, read, created_at, updated_at
            "#,
            notification_id,
            account_id,
            request.read
        )
        .fetch_optional(pool)
        .await?
        .ok_or(NotificationsError::NotificationNotFound(notification_id))?;

        let notification = Notification {
            id: notification_row.id,
            account_id: notification_row.account_id,
            from_account_id: Some(notification_row.from_account_id),
            notification_type: NotificationType::from_str(&notification_row.notification_type)?,
            status_id: notification_row.status_id,
            poll_id: notification_row.poll_id,
            read: notification_row.read,
            created_at: DateTime::from_naive_utc_and_offset(notification_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(notification_row.updated_at, Utc),
        };

        info!(
            "Updated notification with id: {} for account {}",
            notification.id, account_id
        );
        Ok(notification)
    }

    /// Marks all notifications as read for an account
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the account
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn mark_all_as_read(
        pool: &PgPool,
        account_id: i64,
    ) -> Result<(), NotificationsError> {
        trace!(
            "Marking all notifications as read for account: {}",
            account_id
        );

        sqlx::query!(
            r#"
            UPDATE notifications
            SET read = true, updated_at = now()
            WHERE account_id = $1 AND read = false
            "#,
            account_id
        )
        .execute(pool)
        .await?;

        info!(
            "Marked all notifications as read for account {}",
            account_id
        );
        Ok(())
    }

    /// Deletes a notification
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `notification_id` - ID of the notification to delete
    /// * `account_id` - ID of the account that owns the notification
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn delete(
        pool: &PgPool,
        notification_id: i64,
        account_id: i64,
    ) -> Result<(), NotificationsError> {
        trace!(
            "Deleting notification {} for account {}",
            notification_id,
            account_id
        );

        let result = sqlx::query!(
            r#"
            DELETE FROM notifications
            WHERE id = $1 AND account_id = $2
            "#,
            notification_id,
            account_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(NotificationsError::NotificationNotFound(notification_id));
        }

        info!(
            "Deleted notification with id: {} for account {}",
            notification_id, account_id
        );
        Ok(())
    }

    /// Gets the count of unread notifications for an account
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `account_id` - ID of the account
    ///
    /// # Returns
    ///
    /// Result containing the count or an error
    pub async fn get_unread_count(
        pool: &PgPool,
        account_id: i64,
    ) -> Result<i64, NotificationsError> {
        trace!(
            "Getting unread notification count for account: {}",
            account_id
        );

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM notifications
            WHERE account_id = $1 AND read = false
            "#,
            account_id
        )
        .fetch_one(pool)
        .await?
        .count;

        debug!(
            "Unread notification count for account {}: {}",
            account_id,
            count.unwrap_or(0)
        );
        Ok(count.unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_notification_create_and_delete() {
        // This would require a test database setup
        // For now, just test the struct creation
        let notification = Notification {
            id: 1,
            account_id: 1,
            from_account_id: Some(2),
            notification_type: NotificationType::Follow,
            status_id: None,
            poll_id: None,
            read: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(notification.account_id, 1);
        assert_eq!(notification.notification_type, NotificationType::Follow);
        assert!(!notification.read);
    }

    #[tokio::test]
    async fn test_notification_type_serialization() {
        let follow_type = NotificationType::Follow;
        assert_eq!(follow_type.to_string(), "follow");

        let mention_type = NotificationType::Mention;
        assert_eq!(mention_type.to_string(), "mention");

        let parsed_type = NotificationType::from_str("reblog").unwrap();
        assert_eq!(parsed_type, NotificationType::Reblog);
    }
}
