//! Severed relationships functionality for Rustodon
//!
//! This module provides severed relationships capabilities for the Rustodon server.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, info};

/// Severed relationship model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeveredRelationship {
    pub id: i64,
    pub local_account_id: i64,
    pub remote_account_id: i64,
    pub relationship_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Error type for severed relationship operations
#[derive(Error, Debug)]
pub enum SeveredRelationshipError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Relationship error: {0}")]
    Relationship(String),
}

/// Initialize severed relationships functionality
pub async fn init_severed_relationships() -> Result<(), SeveredRelationshipError> {
    info!("Initializing severed relationships functionality");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severed_relationship() {
        let relationship = SeveredRelationship {
            id: 1,
            local_account_id: 1,
            remote_account_id: 2,
            relationship_type: "follow".to_string(),
            created_at: chrono::Utc::now(),
        };
        assert_eq!(relationship.relationship_type, "follow");
    }
}
