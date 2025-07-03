//! List model for Rustodon
//!
//! This module defines the List model and its database operations.
//! It handles user-created lists and list memberships.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use super::user::User;
use crate::error::DbError;
use crate::models::user::UserStatus;
use chrono::NaiveDateTime;
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info, trace};

/// List model representing user-created lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    /// List ID
    pub id: i64,
    /// List title
    pub title: String,
    /// List replies policy (followed, list, none)
    pub replies_policy: Option<String>,
    /// Whether the list is exclusive
    pub exclusive: Option<bool>,
    /// Account ID that owns the list
    pub account_id: i64,
    /// Created at timestamp
    pub created_at: Option<NaiveDateTime>,
}

/// ListAccount model representing list memberships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAccount {
    pub list_id: i64,
    pub account_id: i64,
}

impl List {
    /// Get all lists
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        trace!("Fetching all lists");
        let lists = sqlx::query_as!(
            List,
            "SELECT id, title, replies_policy, exclusive, account_id, created_at FROM lists ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} lists", lists.len());
        Ok(lists)
    }

    /// Get lists for a specific account
    pub async fn get_lists_for_account(
        pool: &PgPool,
        account_id: i64,
    ) -> Result<Vec<Self>, DbError> {
        trace!("Fetching lists for account: {}", account_id);
        let lists = sqlx::query_as!(
            List,
            "SELECT id, title, replies_policy, exclusive, account_id, created_at FROM lists WHERE account_id = $1 ORDER BY created_at DESC",
            account_id
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} lists for account {}", lists.len(), account_id);
        Ok(lists)
    }

    /// Get lists by account (alias for get_lists_for_account)
    pub async fn get_by_account(pool: &PgPool, account_id: i64) -> Result<Vec<Self>, DbError> {
        Self::get_lists_for_account(pool, account_id).await
    }

    /// Get a list by ID
    pub async fn get_by_id(pool: &PgPool, list_id: i64) -> Result<Option<Self>, DbError> {
        trace!("Fetching list by ID: {}", list_id);
        let list = sqlx::query_as!(
            List,
            "SELECT id, title, replies_policy, exclusive, account_id, created_at FROM lists WHERE id = $1",
            list_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(ref list) = list {
            info!("Fetched list: {}", list.title);
        }
        Ok(list)
    }

    /// Create a new list
    pub async fn create(
        pool: &PgPool,
        account_id: i64,
        title: &str,
        replies_policy: Option<&str>,
        exclusive: Option<bool>,
    ) -> Result<Self, DbError> {
        trace!("Creating list: {} for account {}", title, account_id);
        let list = sqlx::query_as!(
            List,
            "INSERT INTO lists (account_id, title, replies_policy, exclusive) VALUES ($1, $2, $3, $4) RETURNING id, title, replies_policy, exclusive, account_id, created_at",
            account_id,
            title,
            replies_policy,
            exclusive
        )
        .fetch_one(pool)
        .await?;

        info!("Created list: {} with ID {}", list.title, list.id);
        Ok(list)
    }

    /// Update list title
    pub async fn update_title(
        pool: &PgPool,
        list_id: i64,
        title: &str,
    ) -> Result<Option<Self>, DbError> {
        trace!("Updating list title: {} to {}", list_id, title);
        let list = sqlx::query_as!(
            List,
            "UPDATE lists SET title = $2 WHERE id = $1 RETURNING id, title, replies_policy, exclusive, account_id, created_at",
            list_id,
            title
        )
        .fetch_optional(pool)
        .await?;

        if list.is_some() {
            info!("Updated list title: {}", list_id);
        } else {
            debug!("List not found for update: {}", list_id);
        }
        Ok(list)
    }

    /// Delete a list
    pub async fn delete(pool: &PgPool, list_id: i64, account_id: i64) -> Result<bool, DbError> {
        trace!("Deleting list: {} for account: {}", list_id, account_id);
        let result = sqlx::query!(
            "DELETE FROM lists WHERE id = $1 AND account_id = $2",
            list_id,
            account_id
        )
        .execute(pool)
        .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            info!("Deleted list: {} for account: {}", list_id, account_id);
        } else {
            debug!(
                "List not found or not owned by account: {} {}",
                list_id, account_id
            );
        }
        Ok(deleted)
    }

    /// Add accounts to a list
    pub async fn add_accounts(
        pool: &PgPool,
        list_id: i64,
        account_ids: &[i64],
    ) -> Result<(), DbError> {
        trace!("Adding {} accounts to list: {}", account_ids.len(), list_id);

        for account_id in account_ids {
            sqlx::query!(
                "INSERT INTO list_accounts (list_id, account_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                list_id,
                account_id
            )
            .execute(pool)
            .await?;
        }

        info!("Added {} accounts to list: {}", account_ids.len(), list_id);
        Ok(())
    }

    /// Remove accounts from a list
    pub async fn remove_accounts(
        pool: &PgPool,
        list_id: i64,
        account_ids: &[i64],
    ) -> Result<(), DbError> {
        trace!(
            "Removing {} accounts from list: {}",
            account_ids.len(),
            list_id
        );

        for account_id in account_ids {
            sqlx::query!(
                "DELETE FROM list_accounts WHERE list_id = $1 AND account_id = $2",
                list_id,
                account_id
            )
            .execute(pool)
            .await?;
        }

        info!(
            "Removed {} accounts from list: {}",
            account_ids.len(),
            list_id
        );
        Ok(())
    }
}

impl ListAccount {
    /// Get all list accounts
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        trace!("Fetching all list accounts");
        let list_accounts =
            sqlx::query_as!(ListAccount, "SELECT list_id, account_id FROM list_accounts")
                .fetch_all(pool)
                .await?;

        info!("Fetched {} list accounts", list_accounts.len());
        Ok(list_accounts)
    }

    /// Get accounts in a list
    pub async fn get_accounts_in_list(pool: &PgPool, list_id: i64) -> Result<Vec<User>, DbError> {
        trace!("Fetching accounts in list: {}", list_id);
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT u.id, u.email, u.username, u.password_hash, u.display_name, u.note, u.status as "status: UserStatus",
                   u.locked, u.bot, u.discoverable, u.group_account, u.avatar_url, u.header_url, u.website, u.location,
                   u.language, u.created_at, u.updated_at, u.last_active_at, u.confirmation_token, u.confirmed_at,
                   u.recovery_email, u.last_status_at, u.statuses_count, u.followers_count, u.following_count,
                   u.remember_created_at, u.sign_in_count, u.current_sign_in_at, u.last_sign_in_at,
                   u.current_sign_in_ip as "current_sign_in_ip: IpNetwork", u.last_sign_in_ip as "last_sign_in_ip: IpNetwork",
                   u.admin, u.moderator, u.approved
            FROM users u
            JOIN list_accounts la ON u.id = la.account_id
            WHERE la.list_id = $1
            ORDER BY u.created_at DESC
            "#,
            list_id
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} accounts in list: {}", users.len(), list_id);
        Ok(users)
    }

    /// Add accounts to a list
    pub async fn add_accounts(
        pool: &PgPool,
        list_id: i64,
        account_ids: &[i64],
    ) -> Result<(), DbError> {
        trace!("Adding {} accounts to list: {}", account_ids.len(), list_id);

        for account_id in account_ids {
            sqlx::query!(
                "INSERT INTO list_accounts (list_id, account_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                list_id,
                account_id
            )
            .execute(pool)
            .await?;
        }

        info!("Added {} accounts to list: {}", account_ids.len(), list_id);
        Ok(())
    }

    /// Remove accounts from a list
    pub async fn remove_accounts(
        pool: &PgPool,
        list_id: i64,
        account_ids: &[i64],
    ) -> Result<(), DbError> {
        trace!(
            "Removing {} accounts from list: {}",
            account_ids.len(),
            list_id
        );

        for account_id in account_ids {
            sqlx::query!(
                "DELETE FROM list_accounts WHERE list_id = $1 AND account_id = $2",
                list_id,
                account_id
            )
            .execute(pool)
            .await?;
        }

        info!(
            "Removed {} accounts from list: {}",
            account_ids.len(),
            list_id
        );
        Ok(())
    }
}
