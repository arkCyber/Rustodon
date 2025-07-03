//! Follow recommendation suppression functionality for Rustodon
//!
//! This module provides follow recommendation suppression capabilities for the Rustodon server.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, error, info, trace, warn};

/// Follow recommendation suppression model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowRecommendationSuppression {
    pub id: i64,
    pub account_id: i64,
    pub target_account_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Error type for follow recommendation suppression operations
#[derive(Error, Debug)]
pub enum FollowRecommendationSuppressionError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Suppression error: {0}")]
    Suppression(String),
}

/// Initialize follow recommendation suppressions functionality
pub async fn init_follow_recommendation_suppressions(
) -> Result<(), FollowRecommendationSuppressionError> {
    info!("Initializing follow recommendation suppressions functionality");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_recommendation_suppression() {
        let suppression = FollowRecommendationSuppression {
            id: 1,
            account_id: 1,
            target_account_id: 2,
            created_at: chrono::Utc::now(),
        };
        assert_eq!(suppression.account_id, 1);
    }
}
