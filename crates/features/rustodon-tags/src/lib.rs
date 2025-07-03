//!
//! Tags module for Rustodon
//!
//! This module provides functionality for managing tags in the Rustodon server.
//! It handles creating, updating, deleting, and querying tags with proper validation
//! and database operations.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_tags::Tag;
//!
//! // Create a tag
//! let tag = Tag::create(&pool, "rustodon").await?;
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
use sqlx::PgPool;
use thiserror::Error;
use tracing::{debug, error, info, trace};

/// Custom error type for tags module
#[derive(Error, Debug)]
pub enum TagError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Tag not found: {0}")]
    TagNotFound(i64),
    #[error("Tag name already exists")]
    TagNameExists,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Tag data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// Unique identifier for the tag
    pub id: i64,
    /// Name of the tag
    pub name: String,
    /// When the tag was created
    pub created_at: DateTime<Utc>,
    /// When the tag was last updated
    pub updated_at: DateTime<Utc>,
}

impl Tag {
    /// Creates a new tag
    pub async fn create(pool: &PgPool, name: &str) -> Result<Self, TagError> {
        trace!("Creating tag: {}", name);
        // Check if tag name already exists
        let exists = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM tags WHERE name = $1"#,
            name
        )
        .fetch_one(pool)
        .await?
        .count;
        if exists > Some(0) {
            return Err(TagError::TagNameExists);
        }
        let tag_row = sqlx::query_as!(
            TagRow,
            r#"
            INSERT INTO tags (name)
            VALUES ($1)
            RETURNING id, name, created_at, updated_at
            "#,
            name
        )
        .fetch_one(pool)
        .await?;
        let tag = Tag {
            id: tag_row.id,
            name: tag_row.name,
            created_at: DateTime::from_naive_utc_and_offset(tag_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(tag_row.updated_at, Utc),
        };
        info!("Created tag: {} (id: {})", tag.name, tag.id);
        Ok(tag)
    }

    /// Gets a tag by id
    pub async fn get(pool: &PgPool, id: i64) -> Result<Self, TagError> {
        trace!("Getting tag by id: {}", id);
        let tag_row = sqlx::query_as!(
            TagRow,
            r#"
            SELECT id, name, created_at, updated_at
            FROM tags
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        let tag_row = tag_row.ok_or(TagError::TagNotFound(id))?;
        let tag = Tag {
            id: tag_row.id,
            name: tag_row.name,
            created_at: DateTime::from_naive_utc_and_offset(tag_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(tag_row.updated_at, Utc),
        };
        debug!("Got tag: {} (id: {})", tag.name, tag.id);
        Ok(tag)
    }

    /// Updates a tag's name
    pub async fn update(pool: &PgPool, id: i64, new_name: &str) -> Result<Self, TagError> {
        trace!("Updating tag id {} to new name: {}", id, new_name);
        // Check if new name already exists
        let exists = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM tags WHERE name = $1 AND id != $2"#,
            new_name,
            id
        )
        .fetch_one(pool)
        .await?
        .count;
        if exists > Some(0) {
            return Err(TagError::TagNameExists);
        }
        let tag_row = sqlx::query_as!(
            TagRow,
            r#"
            UPDATE tags
            SET name = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, name, created_at, updated_at
            "#,
            new_name,
            id
        )
        .fetch_optional(pool)
        .await?;
        let tag_row = tag_row.ok_or(TagError::TagNotFound(id))?;
        let tag = Tag {
            id: tag_row.id,
            name: tag_row.name,
            created_at: DateTime::from_naive_utc_and_offset(tag_row.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(tag_row.updated_at, Utc),
        };
        info!("Updated tag: {} (id: {})", tag.name, tag.id);
        Ok(tag)
    }

    /// Deletes a tag by id
    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), TagError> {
        trace!("Deleting tag by id: {}", id);
        let result = sqlx::query!(r#"DELETE FROM tags WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        if result.rows_affected() == 0 {
            return Err(TagError::TagNotFound(id));
        }
        info!("Deleted tag id: {}", id);
        Ok(())
    }

    /// Lists all tags (with optional limit)
    pub async fn list(pool: &PgPool, limit: Option<i64>) -> Result<Vec<Self>, TagError> {
        trace!("Listing tags with limit: {:?}", limit);
        let limit = limit.unwrap_or(20).min(100);
        let tag_rows = sqlx::query_as!(
            TagRow,
            r#"
            SELECT id, name, created_at, updated_at
            FROM tags
            ORDER BY created_at DESC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(pool)
        .await?;
        let tags: Vec<Tag> = tag_rows
            .into_iter()
            .map(|row| Tag {
                id: row.id,
                name: row.name,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
            })
            .collect();
        debug!("Listed {} tags", tags.len());
        Ok(tags)
    }
}

/// Internal struct for database rows
#[derive(sqlx::FromRow)]
struct TagRow {
    id: i64,
    name: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_tag_struct() {
        let tag = Tag {
            id: 1,
            name: "rustodon".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(tag.name, "rustodon");
    }
    // Note: Full async DB tests would require a test database setup
}
