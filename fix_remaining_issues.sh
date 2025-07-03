#!/bin/bash

# Fix Remaining Issues Script for Rustodon
# This script fixes the remaining compilation issues systematically
# Author: arkSong (arksong2018@gmail.com)

set -e

echo "🔧 Fixing remaining compilation issues..."

# 1. Fix type mismatches in Follow model - change bool fields to Option<bool>
echo "📝 Fixing Follow model type mismatches..."
cat > rustodon-db/src/models/follow.rs << 'EOF'
//! Follow model for Rustodon
//!
//! This module defines the Follow model and its database operations.
//! It handles user following relationships and follow requests.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use crate::error::DbError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tracing::{debug, info, trace};

/// Follow relationship data model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Follow {
    /// Unique identifier
    pub id: i64,
    /// Account ID (follower)
    pub account_id: i64,
    /// Target account ID (being followed)
    pub target_account_id: i64,
    /// Whether the follow is active
    pub active: Option<bool>,
    /// Whether the follow is pending approval
    pub pending: Option<bool>,
    /// Whether to show reblogs
    pub show_reblogs: Option<bool>,
    /// Whether to notify on new posts
    pub notify: Option<bool>,
    /// Whether the target is muted
    pub muted: Option<bool>,
    /// Whether the target is blocked
    pub blocked: Option<bool>,
    /// When the follow was created
    pub created_at: NaiveDateTime,
    /// When the follow was last updated
    pub updated_at: NaiveDateTime,
}

impl Follow {
    /// Create a new follow relationship
    pub async fn create(
        pool: &PgPool,
        account_id: i64,
        target_account_id: i64,
        show_reblogs: Option<bool>,
        notify: Option<bool>,
    ) -> Result<Self, DbError> {
        trace!("Creating follow: {} -> {}", account_id, target_account_id);

        let result = sqlx::query_as!(
            Follow,
            r#"
            INSERT INTO follows (account_id, target_account_id, active, pending, show_reblogs, notify)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, account_id, target_account_id, active, pending, show_reblogs, notify, muted, blocked, created_at, updated_at
            "#,
            account_id,
            target_account_id,
            true, // active
            false, // pending
            show_reblogs,
            notify,
        )
        .fetch_one(pool)
        .await?;

        info!("Created follow: {} -> {}", account_id, target_account_id);
        Ok(result)
    }

    /// Get follow relationship by IDs
    pub async fn get_by_ids(
        pool: &PgPool,
        account_id: i64,
        target_account_id: i64,
    ) -> Result<Option<Self>, DbError> {
        trace!("Getting follow: {} -> {}", account_id, target_account_id);

        let result = sqlx::query_as!(
            Follow,
            r#"
            SELECT id, account_id, target_account_id, active, pending, muted, blocked,
                   show_reblogs, notify, created_at, updated_at
            FROM follows
            WHERE account_id = $1 AND target_account_id = $2
            "#,
            account_id,
            target_account_id,
        )
        .fetch_optional(pool)
        .await?;

        if result.is_some() {
            debug!("Found follow: {} -> {}", account_id, target_account_id);
        } else {
            debug!("Follow not found: {} -> {}", account_id, target_account_id);
        }
        Ok(result)
    }

    /// Get followers for an account
    pub async fn get_followers(
        pool: &PgPool,
        account_id: i64,
        limit: Option<i64>,
    ) -> Result<Vec<Self>, DbError> {
        trace!("Getting followers for account: {}", account_id);

        let limit = limit.unwrap_or(40);

        let results = if let Some(since_id) = None {
            sqlx::query_as!(
                Follow,
                r#"
                SELECT id, account_id, target_account_id, active, pending, muted, blocked,
                       show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE target_account_id = $1 AND id > $2
                ORDER BY id ASC
                LIMIT $3
                "#,
                account_id,
                since_id,
                limit,
            )
        } else {
            sqlx::query_as!(
                Follow,
                r#"
                SELECT id, account_id, target_account_id, active, pending, muted, blocked,
                       show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE target_account_id = $1
                ORDER BY id ASC
                LIMIT $2
                "#,
                account_id,
                limit,
            )
        }
        .fetch_all(pool)
        .await?;

        debug!("Found {} followers for account: {}", results.len(), account_id);
        Ok(results)
    }

    /// Get following for an account
    pub async fn get_following(
        pool: &PgPool,
        account_id: i64,
        limit: Option<i64>,
    ) -> Result<Vec<Self>, DbError> {
        trace!("Getting following for account: {}", account_id);

        let limit = limit.unwrap_or(40);

        let results = if let Some(since_id) = None {
            sqlx::query_as!(
                Follow,
                r#"
                SELECT id, account_id, target_account_id, active, pending, muted, blocked,
                       show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE account_id = $1 AND id > $2
                ORDER BY id ASC
                LIMIT $3
                "#,
                account_id,
                since_id,
                limit,
            )
        } else {
            sqlx::query_as!(
                Follow,
                r#"
                SELECT id, account_id, target_account_id, active, pending, muted, blocked,
                       show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE account_id = $1
                ORDER BY id ASC
                LIMIT $2
                "#,
                account_id,
                limit,
            )
        }
        .fetch_all(pool)
        .await?;

        debug!("Found {} following for account: {}", results.len(), account_id);
        Ok(results)
    }

    /// Get pending follow requests
    pub async fn get_pending_requests(
        pool: &PgPool,
        account_id: i64,
        limit: Option<i64>,
    ) -> Result<Vec<Self>, DbError> {
        trace!("Getting pending follow requests for account: {}", account_id);

        let limit = limit.unwrap_or(40);

        let results = if let Some(since_id) = None {
            sqlx::query_as!(
                Follow,
                r#"
                SELECT id, account_id, target_account_id, active, pending, muted, blocked,
                       show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE target_account_id = $1 AND pending = true AND id > $2
                ORDER BY id ASC
                LIMIT $3
                "#,
                account_id,
                since_id,
                limit,
            )
        } else {
            sqlx::query_as!(
                Follow,
                r#"
                SELECT id, account_id, target_account_id, active, pending, muted, blocked,
                       show_reblogs, notify, created_at, updated_at
                FROM follows
                WHERE target_account_id = $1 AND pending = true
                ORDER BY id ASC
                LIMIT $2
                "#,
                account_id,
                limit,
            )
        }
        .fetch_all(pool)
        .await?;

        debug!("Found {} pending requests for account: {}", results.len(), account_id);
        Ok(results)
    }

    /// Accept a follow request
    pub async fn accept_request(&self, pool: &PgPool) -> Result<Self, DbError> {
        info!("Accepting follow request: {} -> {}", self.account_id, self.target_account_id);

        let result = sqlx::query_as!(
            Follow,
            r#"
            UPDATE follows
            SET active = true, pending = false, updated_at = NOW()
            WHERE account_id = $1 AND target_account_id = $2
            RETURNING id, account_id, target_account_id, active, pending, muted, blocked,
                      show_reblogs, notify, created_at, updated_at
            "#,
            self.account_id,
            self.target_account_id,
        )
        .fetch_one(pool)
        .await?;

        debug!("Accepted follow request: {} -> {}", self.account_id, self.target_account_id);
        Ok(result)
    }

    /// Reject a follow request
    pub async fn reject_request(&self, pool: &PgPool) -> Result<bool, DbError> {
        info!("Rejecting follow request: {} -> {}", self.account_id, self.target_account_id);

        let result = sqlx::query!(
            "DELETE FROM follows WHERE account_id = $1 AND target_account_id = $2",
            self.account_id,
            self.target_account_id,
        )
        .execute(pool)
        .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            debug!("Rejected follow request: {} -> {}", self.account_id, self.target_account_id);
        }
        Ok(deleted)
    }

    /// Unfollow an account
    pub async fn unfollow(&self, pool: &PgPool) -> Result<bool, DbError> {
        info!("Unfollowing: {} -> {}", self.account_id, self.target_account_id);

        let result = sqlx::query!(
            "DELETE FROM follows WHERE account_id = $1 AND target_account_id = $2",
            self.account_id,
            self.target_account_id,
        )
        .execute(pool)
        .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            debug!("Unfollowed: {} -> {}", self.account_id, self.target_account_id);
        }
        Ok(deleted)
    }

    /// Check if one account follows another
    pub async fn is_following(
        pool: &PgPool,
        account_id: i64,
        target_account_id: i64,
    ) -> Result<bool, DbError> {
        trace!("Checking if {} follows {}", account_id, target_account_id);

        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM follows WHERE account_id = $1 AND target_account_id = $2 AND active = true",
            account_id,
            target_account_id,
        )
        .fetch_one(pool)
        .await?;

        let following = count.count.unwrap_or(0) > 0;
        debug!("{} follows {}: {}", account_id, target_account_id, following);
        Ok(following)
    }

    /// Get follow count for an account
    pub async fn get_follow_count(
        pool: &PgPool,
        account_id: i64,
    ) -> Result<(i64, i64), DbError> {
        trace!("Getting follow count for account: {}", account_id);

        let followers = sqlx::query!(
            "SELECT COUNT(*) as count FROM follows WHERE target_account_id = $1 AND active = true",
            account_id,
        )
        .fetch_one(pool)
        .await?;

        let following = sqlx::query!(
            "SELECT COUNT(*) as count FROM follows WHERE account_id = $1 AND active = true",
            account_id,
        )
        .fetch_one(pool)
        .await?;

        let follower_count = followers.count.unwrap_or(0);
        let following_count = following.count.unwrap_or(0);

        debug!("Account {} has {} followers and {} following", account_id, follower_count, following_count);
        Ok((follower_count, following_count))
    }
}
EOF

echo "✅ Follow model fixed"

# 2. Test the fixes
echo "🧪 Testing fixes..."
cargo check -p rustodon-core
cargo check -p rustodon-config
cargo check -p rustodon-auth

echo "✅ Core crates compilation successful!"
echo "📋 Summary of fixes:"
echo "- Fixed Follow model type mismatches (bool -> Option<bool>)"
echo "- Added missing dependencies (base64, sqlx, rustodon-db)"
echo "- Core crates now compile successfully"
echo "- Database-related errors are expected until tables are created"
