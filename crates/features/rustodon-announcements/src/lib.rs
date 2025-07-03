//! Announcements module for Rustodon
//!
//! This module provides functionality for managing announcements in the Rustodon server.
//! It handles creating, updating, and retrieving announcements with proper validation
//! and database operations.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_announcements::{Announcement, AnnouncementService};
//!
//! let service = AnnouncementService::new(pool);
//! let announcements = service.get_all_published().await?;
//! ```
//!
//! # Dependencies
//!
//! - `rustodon_core`: Core types and traits
//! - `rustodon_db`: Database operations
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use thiserror::Error;
use tracing::{debug, error, info, trace};

/// Custom error type for announcements module
#[derive(Error, Debug)]
pub enum AnnouncementError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Announcement not found: {0}")]
    NotFound(i64),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Announcement data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    /// Unique identifier
    pub id: i64,
    /// Announcement text content
    pub text: String,
    /// Whether the announcement is published
    pub published: bool,
    /// Whether the announcement is all day
    pub all_day: bool,
    /// Scheduled start time
    pub scheduled_at: Option<DateTime<Utc>>,
    /// Start time
    pub starts_at: Option<DateTime<Utc>>,
    /// End time
    pub ends_at: Option<DateTime<Utc>>,
    /// Whether the announcement is published
    pub published_at: Option<DateTime<Utc>>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

impl Announcement {
    /// Creates a new announcement instance
    ///
    /// # Arguments
    ///
    /// * `text` - The announcement text content
    /// * `published` - Whether the announcement is published
    /// * `all_day` - Whether the announcement is all day
    ///
    /// # Returns
    ///
    /// A new Announcement instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustodon_announcements::Announcement;
    ///
    /// let announcement = Announcement::new("Important update", true, false);
    /// assert_eq!(announcement.text, "Important update");
    /// ```
    pub fn new(text: impl Into<String>, published: bool, all_day: bool) -> Self {
        let text = text.into();
        let now = Utc::now();
        trace!("Creating new announcement with text: {}", text);

        Self {
            id: 0,
            text,
            published,
            all_day,
            scheduled_at: None,
            starts_at: None,
            ends_at: None,
            published_at: if published { Some(now) } else { None },
            created_at: now,
            updated_at: now,
        }
    }

    /// Validates the announcement data
    ///
    /// # Returns
    ///
    /// Result indicating if the announcement is valid
    pub fn validate(&self) -> Result<(), AnnouncementError> {
        trace!("Validating announcement: {:?}", self);

        if self.text.trim().is_empty() {
            return Err(AnnouncementError::Validation(
                "Text cannot be empty".to_string(),
            ));
        }

        if self.text.len() > 1000 {
            return Err(AnnouncementError::Validation(
                "Text too long (max 1000 characters)".to_string(),
            ));
        }

        if let (Some(starts_at), Some(ends_at)) = (self.starts_at, self.ends_at) {
            if starts_at >= ends_at {
                return Err(AnnouncementError::Validation(
                    "Start time must be before end time".to_string(),
                ));
            }
        }

        debug!("Announcement validation passed");
        Ok(())
    }
}

/// Service for managing announcements
pub struct AnnouncementService {
    pool: PgPool,
}

impl AnnouncementService {
    /// Creates a new announcement service
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    ///
    /// # Returns
    ///
    /// A new AnnouncementService instance
    pub fn new(pool: PgPool) -> Self {
        info!("Creating new AnnouncementService");
        Self { pool }
    }

    /// Creates a new announcement in the database
    ///
    /// # Arguments
    ///
    /// * `announcement` - The announcement to create
    ///
    /// # Returns
    ///
    /// The created announcement with ID
    pub async fn create(
        &self,
        announcement: Announcement,
    ) -> Result<Announcement, AnnouncementError> {
        info!("Creating new announcement: {:?}", announcement);

        announcement.validate()?;

        let query = r#"
            INSERT INTO announcements (text, published, all_day, scheduled_at, starts_at, ends_at, published_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, text, published, all_day, scheduled_at, starts_at, ends_at, published_at, created_at, updated_at
        "#;

        let row = sqlx::query(query)
            .bind(&announcement.text)
            .bind(announcement.published)
            .bind(announcement.all_day)
            .bind(announcement.scheduled_at)
            .bind(announcement.starts_at)
            .bind(announcement.ends_at)
            .bind(announcement.published_at)
            .bind(announcement.created_at)
            .bind(announcement.updated_at)
            .fetch_one(&self.pool)
            .await?;

        let created_announcement = Announcement {
            id: row.get("id"),
            text: row.get("text"),
            published: row.get("published"),
            all_day: row.get("all_day"),
            scheduled_at: row.get("scheduled_at"),
            starts_at: row.get("starts_at"),
            ends_at: row.get("ends_at"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        info!("Created announcement with ID: {}", created_announcement.id);
        Ok(created_announcement)
    }

    /// Retrieves an announcement by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The announcement ID
    ///
    /// # Returns
    ///
    /// The announcement if found
    pub async fn get_by_id(&self, id: i64) -> Result<Option<Announcement>, AnnouncementError> {
        trace!("Getting announcement by ID: {}", id);

        let query = r#"
            SELECT id, text, published, all_day, scheduled_at, starts_at, ends_at, published_at, created_at, updated_at
            FROM announcements
            WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => {
                let announcement = Announcement {
                    id: row.get("id"),
                    text: row.get("text"),
                    published: row.get("published"),
                    all_day: row.get("all_day"),
                    scheduled_at: row.get("scheduled_at"),
                    starts_at: row.get("starts_at"),
                    ends_at: row.get("ends_at"),
                    published_at: row.get("published_at"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                debug!("Found announcement: {:?}", announcement);
                Ok(Some(announcement))
            }
            None => {
                debug!("Announcement not found with ID: {}", id);
                Ok(None)
            }
        }
    }

    /// Retrieves all published announcements
    ///
    /// # Returns
    ///
    /// List of published announcements
    pub async fn get_all_published(&self) -> Result<Vec<Announcement>, AnnouncementError> {
        info!("Getting all published announcements");

        let query = r#"
            SELECT id, text, published, all_day, scheduled_at, starts_at, ends_at, published_at, created_at, updated_at
            FROM announcements
            WHERE published = true
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        let announcements: Vec<Announcement> = rows
            .into_iter()
            .map(|row| Announcement {
                id: row.get("id"),
                text: row.get("text"),
                published: row.get("published"),
                all_day: row.get("all_day"),
                scheduled_at: row.get("scheduled_at"),
                starts_at: row.get("starts_at"),
                ends_at: row.get("ends_at"),
                published_at: row.get("published_at"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        info!("Retrieved {} published announcements", announcements.len());
        Ok(announcements)
    }

    /// Updates an announcement
    ///
    /// # Arguments
    ///
    /// * `id` - The announcement ID
    /// * `announcement` - The updated announcement data
    ///
    /// # Returns
    ///
    /// The updated announcement
    pub async fn update(
        &self,
        id: i64,
        mut announcement: Announcement,
    ) -> Result<Announcement, AnnouncementError> {
        info!("Updating announcement with ID: {}", id);

        announcement.validate()?;
        announcement.updated_at = Utc::now();

        let query = r#"
            UPDATE announcements
            SET text = $1, published = $2, all_day = $3, scheduled_at = $4, starts_at = $5, ends_at = $6, published_at = $7, updated_at = $8
            WHERE id = $9
            RETURNING id, text, published, all_day, scheduled_at, starts_at, ends_at, published_at, created_at, updated_at
        "#;

        let row = sqlx::query(query)
            .bind(&announcement.text)
            .bind(announcement.published)
            .bind(announcement.all_day)
            .bind(announcement.scheduled_at)
            .bind(announcement.starts_at)
            .bind(announcement.ends_at)
            .bind(announcement.published_at)
            .bind(announcement.updated_at)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        let updated_announcement = Announcement {
            id: row.get("id"),
            text: row.get("text"),
            published: row.get("published"),
            all_day: row.get("all_day"),
            scheduled_at: row.get("scheduled_at"),
            starts_at: row.get("starts_at"),
            ends_at: row.get("ends_at"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        info!("Updated announcement with ID: {}", updated_announcement.id);
        Ok(updated_announcement)
    }

    /// Deletes an announcement
    ///
    /// # Arguments
    ///
    /// * `id` - The announcement ID
    ///
    /// # Returns
    ///
    /// Result indicating success
    pub async fn delete(&self, id: i64) -> Result<(), AnnouncementError> {
        info!("Deleting announcement with ID: {}", id);

        let query = "DELETE FROM announcements WHERE id = $1";
        let result = sqlx::query(query).bind(id).execute(&self.pool).await?;

        if result.rows_affected() == 0 {
            return Err(AnnouncementError::NotFound(id));
        }

        info!("Deleted announcement with ID: {}", id);
        Ok(())
    }

    /// Publishes an announcement
    ///
    /// # Arguments
    ///
    /// * `id` - The announcement ID
    ///
    /// # Returns
    ///
    /// The published announcement
    pub async fn publish(&self, id: i64) -> Result<Announcement, AnnouncementError> {
        info!("Publishing announcement with ID: {}", id);

        let now = Utc::now();
        let query = r#"
            UPDATE announcements
            SET published = true, published_at = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, text, published, all_day, scheduled_at, starts_at, ends_at, published_at, created_at, updated_at
        "#;

        let row = sqlx::query(query)
            .bind(now)
            .bind(now)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        let published_announcement = Announcement {
            id: row.get("id"),
            text: row.get("text"),
            published: row.get("published"),
            all_day: row.get("all_day"),
            scheduled_at: row.get("scheduled_at"),
            starts_at: row.get("starts_at"),
            ends_at: row.get("ends_at"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        info!(
            "Published announcement with ID: {}",
            published_announcement.id
        );
        Ok(published_announcement)
    }

    /// Unpublishes an announcement
    ///
    /// # Arguments
    ///
    /// * `id` - The announcement ID
    ///
    /// # Returns
    ///
    /// The unpublished announcement
    pub async fn unpublish(&self, id: i64) -> Result<Announcement, AnnouncementError> {
        info!("Unpublishing announcement with ID: {}", id);

        let now = Utc::now();
        let query = r#"
            UPDATE announcements
            SET published = false, published_at = NULL, updated_at = $1
            WHERE id = $2
            RETURNING id, text, published, all_day, scheduled_at, starts_at, ends_at, published_at, created_at, updated_at
        "#;

        let row = sqlx::query(query)
            .bind(now)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        let unpublished_announcement = Announcement {
            id: row.get("id"),
            text: row.get("text"),
            published: row.get("published"),
            all_day: row.get("all_day"),
            scheduled_at: row.get("scheduled_at"),
            starts_at: row.get("starts_at"),
            ends_at: row.get("ends_at"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        info!(
            "Unpublished announcement with ID: {}",
            unpublished_announcement.id
        );
        Ok(unpublished_announcement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_announcement_new() {
        let announcement = Announcement::new("Test announcement", true, false);
        assert_eq!(announcement.text, "Test announcement");
        assert!(announcement.published);
        assert!(!announcement.all_day);
    }

    #[tokio::test]
    async fn test_announcement_validation() {
        let mut announcement = Announcement::new("Valid text", true, false);
        assert!(announcement.validate().is_ok());

        announcement.text = "".to_string();
        assert!(announcement.validate().is_err());

        announcement.text = "a".repeat(1001);
        assert!(announcement.validate().is_err());
    }

    #[tokio::test]
    async fn test_announcement_validation_time_range() {
        let mut announcement = Announcement::new("Test", true, false);
        announcement.starts_at = Some(Utc::now());
        announcement.ends_at = Some(Utc::now() - chrono::Duration::hours(1));
        assert!(announcement.validate().is_err());
    }

    #[tokio::test]
    async fn test_announcement_service_create() {
        // This would require a test database setup
        // For now, just test the structure
        let announcement = Announcement::new("Test", true, false);
        assert_eq!(announcement.text, "Test");
    }
}
