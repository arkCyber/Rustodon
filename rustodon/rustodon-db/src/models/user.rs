//! User model for Rustodon
//!
//! This module defines the User model and its database operations.
//! It handles user account management and profile data.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use crate::error::DbError;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tracing::{debug, info, trace};
use ipnetwork::IpNetwork;

/// User account status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Suspended,
    Deleted,
    Unconfirmed,
}

/// User data model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// Unique identifier
    pub id: i64,
    /// Username (unique)
    pub username: String,
    /// Email address (unique)
    pub email: String,
    /// Hashed password
    pub password_hash: String,
    /// Display name
    pub display_name: Option<String>,
    /// Bio/note
    pub note: Option<String>,
    /// Account status
    pub status: Option<UserStatus>,
    /// Whether account is locked
    pub locked: Option<bool>,
    /// Whether account is a bot
    pub bot: Option<bool>,
    /// Whether account is discoverable
    pub discoverable: Option<bool>,
    /// Whether account is a group
    pub group_account: Option<bool>,
    /// Avatar URL
    pub avatar_url: Option<String>,
    /// Header URL
    pub header_url: Option<String>,
    /// Website URL
    pub website: Option<String>,
    /// Location
    pub location: Option<String>,
    /// Language
    pub language: Option<String>,
    /// When the account was created
    pub created_at: NaiveDateTime,
    /// When the account was last updated
    pub updated_at: NaiveDateTime,
    /// When the account was last active
    pub last_active_at: Option<NaiveDateTime>,
    /// Email confirmation token
    pub confirmation_token: Option<String>,
    /// When email was confirmed
    pub confirmed_at: Option<NaiveDateTime>,
    /// Recovery email
    pub recovery_email: Option<String>,
    /// Last status timestamp
    pub last_status_at: Option<NaiveDateTime>,
    /// Status count
    pub statuses_count: Option<i64>,
    /// Followers count
    pub followers_count: Option<i64>,
    /// Following count
    pub following_count: Option<i64>,
    /// Remember created at
    pub remember_created_at: Option<NaiveDateTime>,
    /// Sign in count
    pub sign_in_count: Option<i32>,
    /// Current sign in at
    pub current_sign_in_at: Option<NaiveDateTime>,
    /// Last sign in at
    pub last_sign_in_at: Option<NaiveDateTime>,
    /// Current sign in IP
    pub current_sign_in_ip: Option<IpNetwork>,
    /// Last sign in IP
    pub last_sign_in_ip: Option<IpNetwork>,
    /// Whether user is admin
    pub admin: Option<bool>,
    /// Whether user is moderator
    pub moderator: Option<bool>,
    /// Whether user is approved
    pub approved: Option<bool>,
}

impl User {
    /// Get all users
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, DbError> {
        trace!("Fetching all users");
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, note, status as "status: UserStatus",
                   locked, bot, discoverable, group_account, avatar_url, header_url, website, location,
                   language, created_at, updated_at, last_active_at, confirmation_token, confirmed_at,
                   recovery_email, last_status_at, statuses_count, followers_count, following_count,
                   remember_created_at, sign_in_count, current_sign_in_at, last_sign_in_at, current_sign_in_ip,
                   last_sign_in_ip, admin, moderator, approved
            FROM users ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        info!("Fetched {} users", users.len());
        Ok(users)
    }

    /// Get user by ID
    pub async fn get_by_id(pool: &PgPool, user_id: i64) -> Result<Option<Self>, DbError> {
        trace!("Fetching user by ID: {}", user_id);
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, note, status as "status: UserStatus",
                   locked, bot, discoverable, group_account, avatar_url, header_url, website, location,
                   language, created_at, updated_at, last_active_at, confirmation_token, confirmed_at,
                   recovery_email, last_status_at, statuses_count, followers_count, following_count,
                   remember_created_at, sign_in_count, current_sign_in_at, last_sign_in_at, current_sign_in_ip,
                   last_sign_in_ip, admin, moderator, approved
            FROM users WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if user.is_some() {
            debug!("Found user: {}", user_id);
        } else {
            debug!("User not found: {}", user_id);
        }
        Ok(user)
    }

    /// Get user by username
    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Option<Self>, DbError> {
        trace!("Fetching user by username: {}", username);
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, note, status as "status: UserStatus",
                   locked, bot, discoverable, group_account, avatar_url, header_url, website, location,
                   language, created_at, updated_at, last_active_at, confirmation_token, confirmed_at,
                   recovery_email, last_status_at, statuses_count, followers_count, following_count,
                   remember_created_at, sign_in_count, current_sign_in_at, last_sign_in_at, current_sign_in_ip,
                   last_sign_in_ip, admin, moderator, approved
            FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        if user.is_some() {
            debug!("Found user with username: {}", username);
        } else {
            debug!("User not found with username: {}", username);
        }
        Ok(user)
    }

    /// Get user by email
    pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>, DbError> {
        trace!("Fetching user by email: {}", email);
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, note, status as "status: UserStatus",
                   locked, bot, discoverable, group_account, avatar_url, header_url, website, location,
                   language, created_at, updated_at, last_active_at, confirmation_token, confirmed_at,
                   recovery_email, last_status_at, statuses_count, followers_count, following_count,
                   remember_created_at, sign_in_count, current_sign_in_at, last_sign_in_at, current_sign_in_ip,
                   last_sign_in_ip, admin, moderator, approved
            FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        if user.is_some() {
            debug!("Found user with email: {}", email);
        } else {
            debug!("User not found with email: {}", email);
        }
        Ok(user)
    }

    /// Create a new user
    pub async fn create(
        pool: &PgPool,
        email: &str,
        username: &str,
        password: &str,
        display_name: Option<&str>,
    ) -> Result<Self, DbError> {
        trace!("Creating new user with username: {}", username);
        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|e| DbError::Internal(format!("Password hashing failed: {}", e)))?;

        let confirmation_token = Self::generate_token();

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, username, password_hash, display_name, confirmation_token, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, username, email, password_hash, display_name, note, status as "status: UserStatus",
                      locked, bot, discoverable, group_account, avatar_url, header_url, website, location,
                      language, created_at, updated_at, last_active_at, confirmation_token, confirmed_at,
                      recovery_email, last_status_at, statuses_count, followers_count, following_count,
                      remember_created_at, sign_in_count, current_sign_in_at, last_sign_in_at, current_sign_in_ip,
                      last_sign_in_ip, admin, moderator, approved
            "#,
            email,
            username,
            password_hash,
            display_name,
            confirmation_token,
            UserStatus::Unconfirmed as UserStatus,
        )
        .fetch_one(pool)
        .await?;

        info!("Created new user: {}", username);
        Ok(user)
    }

    /// Update user profile
    pub async fn update_profile(
        pool: &PgPool,
        user_id: i64,
        display_name: Option<&str>,
        note: Option<&str>,
        avatar_url: Option<&str>,
        header_url: Option<&str>,
    ) -> Result<Option<Self>, DbError> {
        trace!("Updating user profile: {}", user_id);
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET display_name = COALESCE($2, display_name),
                note = COALESCE($3, note),
                avatar_url = COALESCE($4, avatar_url),
                header_url = COALESCE($5, header_url)
            WHERE id = $1
            RETURNING id, username, email, password_hash, display_name, note, status as "status: UserStatus",
                      locked, bot, discoverable, group_account, avatar_url, header_url, website, location,
                      language, created_at, updated_at, last_active_at, confirmation_token, confirmed_at,
                      recovery_email, last_status_at, statuses_count, followers_count, following_count,
                      remember_created_at, sign_in_count, current_sign_in_at, last_sign_in_at, current_sign_in_ip,
                      last_sign_in_ip, admin, moderator, approved
            "#,
            user_id,
            display_name,
            note,
            avatar_url,
            header_url
        )
        .fetch_optional(pool)
        .await?;

        if user.is_some() {
            info!("Updated user profile: {}", user_id);
        } else {
            debug!("User not found for update: {}", user_id);
        }
        Ok(user)
    }

    /// Delete user
    pub async fn delete(pool: &PgPool, user_id: i64) -> Result<bool, DbError> {
        trace!("Deleting user: {}", user_id);
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(pool)
            .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            info!("Deleted user: {}", user_id);
        } else {
            debug!("User not found for deletion: {}", user_id);
        }
        Ok(deleted)
    }

    /// Check if username exists
    pub async fn username_exists(pool: &PgPool, username: &str) -> Result<bool, DbError> {
        trace!("Checking if username exists: {}", username);
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM users WHERE username = $1",
            username
        )
        .fetch_one(pool)
        .await?;

        let exists = count.count.unwrap_or(0) > 0;
        debug!("Username exists: {} = {}", username, exists);
        Ok(exists)
    }

    /// Check if email exists
    pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, DbError> {
        trace!("Checking if email exists: {}", email);
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM users WHERE email = $1",
            email
        )
        .fetch_one(pool)
        .await?;

        let exists = count.count.unwrap_or(0) > 0;
        debug!("Email exists: {} = {}", email, exists);
        Ok(exists)
    }

    /// Authenticate a user with username/email and password
    pub async fn authenticate(
        pool: &PgPool,
        identifier: &str,
        password: &str,
    ) -> Result<Option<Self>, DbError> {
        debug!("Authenticating user: {}", identifier);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, note, status as "status: UserStatus",
                   locked, bot, discoverable, group_account, avatar_url, header_url, website, location, language,
                   created_at, updated_at, last_active_at, confirmation_token, confirmed_at, recovery_email,
                   last_status_at, statuses_count, followers_count, following_count, remember_created_at,
                   sign_in_count, current_sign_in_at, last_sign_in_at, current_sign_in_ip, last_sign_in_ip,
                   admin, moderator, approved
            FROM users
            WHERE (username = $1 OR email = $1) AND status = 'active'
            "#,
            identifier,
        )
        .fetch_optional(pool)
        .await?;

        if let Some(user) = user {
            if verify(password, &user.password_hash)
                .map_err(|e| DbError::Internal(format!("Password verification failed: {}", e)))?
            {
                let _ = sqlx::query!(
                    r#"
                    UPDATE users
                    SET last_active_at = NOW()
                    WHERE id = $1
                    "#,
                    user.id,
                )
                .execute(pool)
                .await;

                debug!("User authenticated successfully: {}", user.username);
                Ok(Some(user))
            } else {
                debug!("Invalid password for user: {}", identifier);
                Ok(None)
            }
        } else {
            debug!("User not found: {}", identifier);
            Ok(None)
        }
    }

    /// Change user password
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `new_password` - New plain text password
    ///
    /// # Returns
    ///
    /// The updated user
    ///
    /// # Examples
    ///
    /// ```rust
    /// let user = user.change_password(&pool, "newpassword123").await?;
    /// ```
    pub async fn change_password(
        &self,
        pool: &PgPool,
        new_password: &str,
    ) -> Result<Self, DbError> {
        info!("Changing password for user: {}", self.id);

        // Hash the new password
        let password_hash = hash(new_password, DEFAULT_COST)
            .map_err(|e| DbError::Internal(format!("Password hashing failed: {}", e)))?;

        let result = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET password_hash = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, username, email, password_hash, display_name, note, status as "status: UserStatus",
                      locked, bot, discoverable, group_account, avatar_url, header_url, website, location, language,
                      created_at, updated_at, last_active_at, confirmation_token, confirmed_at, recovery_email,
                      last_status_at, statuses_count, followers_count, following_count, remember_created_at,
                      sign_in_count, current_sign_in_at, last_sign_in_at, current_sign_in_ip, last_sign_in_ip,
                      admin, moderator, approved
            "#,
            password_hash,
            self.id,
        )
        .fetch_one(pool)
        .await?;

        debug!("Changed password for user: {}", result.id);
        Ok(result)
    }

    /// Confirm email address
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `token` - Confirmation token
    ///
    /// # Returns
    ///
    /// True if email was confirmed, false if token is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// let confirmed = User::confirm_email(&pool, "token123").await?;
    /// ```
    pub async fn confirm_email(pool: &PgPool, token: &str) -> Result<bool, DbError> {
        debug!("Confirming email with token: {}", token);

        let result = sqlx::query!(
            r#"
            UPDATE users
            SET status = 'active', confirmed_at = NOW(), confirmation_token = NULL
            WHERE confirmation_token = $1 AND status = 'unconfirmed'
            "#,
            token,
        )
        .execute(pool)
        .await?;

        let confirmed = result.rows_affected() > 0;
        if confirmed {
            info!("Email confirmed with token: {}", token);
        } else {
            debug!("Invalid or expired confirmation token: {}", token);
        }

        Ok(confirmed)
    }

    /// Update account status
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool
    /// * `status` - New account status
    ///
    /// # Returns
    ///
    /// The updated user
    ///
    /// # Examples
    ///
    /// ```rust
    /// let user = user.update_status(&pool, UserStatus::Suspended).await?;
    /// ```
    pub async fn update_status(&self, pool: &PgPool, status: UserStatus) -> Result<Self, DbError> {
        info!("Updating status for user {} to {:?}", self.id, status);

        let result = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, username, email, password_hash, display_name, note, status as "status: UserStatus",
                      locked, bot, discoverable, group_account, avatar_url, header_url, website, location, language,
                      created_at, updated_at, last_active_at, confirmation_token, confirmed_at, recovery_email,
                      last_status_at, statuses_count, followers_count, following_count, remember_created_at,
                      sign_in_count, current_sign_in_at, last_sign_in_at, current_sign_in_ip, last_sign_in_ip,
                      admin, moderator, approved
            "#,
            status as UserStatus,
            self.id,
        )
        .fetch_one(pool)
        .await?;

        debug!("Updated status for user: {}", result.id);
        Ok(result)
    }

    /// Check if user is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, Some(UserStatus::Active))
    }

    /// Check if user is confirmed
    pub fn is_confirmed(&self) -> bool {
        self.confirmed_at.is_some()
    }

    /// Generate a random token
    fn generate_token() -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let token: String = (0..32).map(|_| rng.sample(Alphanumeric) as char).collect();

        format!("token_{}", token)
    }
}
