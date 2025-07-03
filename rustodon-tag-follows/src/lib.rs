//! Tag following functionality for Rustodon
//!
//! This module provides tag following capabilities for the Rustodon server.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, error, info, trace, warn};

/// Tag follow model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagFollow {
    pub id: i64,
    pub account_id: i64,
    pub tag_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Error type for tag follow operations
#[derive(Error, Debug)]
pub enum TagFollowError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Tag follow error: {0}")]
    TagFollow(String),
}

/// Initialize tag follows functionality
pub async fn init_tag_follows() -> Result<(), TagFollowError> {
    info!("Initializing tag follows functionality");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_follow() {
        let tag_follow = TagFollow {
            id: 1,
            account_id: 1,
            tag_id: 1,
            created_at: chrono::Utc::now(),
        };
        assert_eq!(tag_follow.account_id, 1);
    }
}
