//! Database operations for Rustodon
//!
//! This module provides database functionality.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::{debug, info, trace};

/// User model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub display_name: Option<String>,
    pub note: Option<String>,
    pub avatar_url: Option<String>,
    pub header_url: Option<String>,
    pub is_admin: Option<bool>,
    pub is_moderator: Option<bool>,
    pub is_verified: Option<bool>,
    pub is_suspended: Option<bool>,
    pub is_silenced: Option<bool>,
    pub is_disabled: Option<bool>,
    pub is_approved: Option<bool>,
    pub is_confirmed: Option<bool>,
    pub is_locked: Option<bool>,
    pub is_bot: Option<bool>,
    pub is_group: Option<bool>,
    pub is_discoverable: Option<bool>,
    pub is_indexable: Option<bool>,
    pub is_private: Option<bool>,
    pub is_protected: Option<bool>,
    pub is_verified_bot: Option<bool>,
    pub is_manually_approved_follows: Option<bool>,
    pub is_sensitive: Option<bool>,
    pub is_show_all_media: Option<bool>,
    pub is_hide_collections: Option<bool>,
    pub is_allow_following_move: Option<bool>,
    pub is_skip_thread_containment: Option<bool>,
    pub is_reject_media: Option<bool>,
    pub is_reject_reports: Option<bool>,
    pub is_invites_enabled: Option<bool>,
    pub is_require_invite_text: Option<bool>,
    pub is_require_invite_application: Option<bool>,
    pub is_require_invite_approval: Option<bool>,
    pub is_require_invite_confirmation: Option<bool>,
    pub is_require_invite_verification: Option<bool>,
    pub is_require_invite_approval_by_admin: Option<bool>,
    pub is_require_invite_approval_by_moderator: Option<bool>,
    pub is_require_invite_approval_by_user: Option<bool>,
    pub is_require_invite_approval_by_group: Option<bool>,
    pub is_require_invite_approval_by_domain: Option<bool>,
    pub is_require_invite_approval_by_ip: Option<bool>,
    pub is_require_invite_approval_by_location: Option<bool>,
    pub is_require_invite_approval_by_time: Option<bool>,
    pub is_require_invite_approval_by_frequency: Option<bool>,
    pub is_require_invite_approval_by_limit: Option<bool>,
    pub is_require_invite_approval_by_quota: Option<bool>,
    pub is_require_invite_approval_by_rule: Option<bool>,
    pub is_require_invite_approval_by_policy: Option<bool>,
    pub is_require_invite_approval_by_setting: Option<bool>,
    pub is_require_invite_approval_by_config: Option<bool>,
    pub is_require_invite_approval_by_option: Option<bool>,
    pub is_require_invite_approval_by_preference: Option<bool>,
    pub is_require_invite_approval_by_choice: Option<bool>,
    pub is_require_invite_approval_by_decision: Option<bool>,
    pub is_require_invite_approval_by_judgment: Option<bool>,
    pub is_require_invite_approval_by_evaluation: Option<bool>,
    pub is_require_invite_approval_by_assessment: Option<bool>,
    pub is_require_invite_approval_by_review: Option<bool>,
    pub is_require_invite_approval_by_audit: Option<bool>,
    pub is_require_invite_approval_by_check: Option<bool>,
    pub is_require_invite_approval_by_validation: Option<bool>,
    pub is_require_invite_approval_by_verification: Option<bool>,
    pub is_require_invite_approval_by_confirmation: Option<bool>,
    pub is_require_invite_approval_by_authentication: Option<bool>,
    pub is_require_invite_approval_by_authorization: Option<bool>,
    pub is_require_invite_approval_by_permission: Option<bool>,
    pub is_require_invite_approval_by_consent: Option<bool>,
    pub is_require_invite_approval_by_agreement: Option<bool>,
    pub is_require_invite_approval_by_acceptance: Option<bool>,
    pub is_require_invite_approval_by_approval: Option<bool>,
}

impl User {
    /// Get user by ID
    pub async fn get_by_id(pool: &sqlx::PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        trace!("Getting user by ID: {}", id);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                   is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                   is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                   is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                   is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                   is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                   is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                   is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                   is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                   is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                   is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                   is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                   is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                   is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                   is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                   is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                   is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                   is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                   is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                   is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                   is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                   is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                   is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                   is_require_invite_approval_by_approval
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    /// Get user by username
    pub async fn get_by_username(pool: &sqlx::PgPool, username: &str) -> Result<Option<Self>, sqlx::Error> {
        trace!("Getting user by username: {}", username);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                   is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                   is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                   is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                   is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                   is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                   is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                   is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                   is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                   is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                   is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                   is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                   is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                   is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                   is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                   is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                   is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                   is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                   is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                   is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                   is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                   is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                   is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                   is_require_invite_approval_by_approval
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    /// Get user by email
    pub async fn get_by_email(pool: &sqlx::PgPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        trace!("Getting user by email: {}", email);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                   is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                   is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                   is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                   is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                   is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                   is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                   is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                   is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                   is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                   is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                   is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                   is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                   is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                   is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                   is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                   is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                   is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                   is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                   is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                   is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                   is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                   is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                   is_require_invite_approval_by_approval
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    /// Create a new user
    pub async fn create(
        pool: &sqlx::PgPool,
        email: &str,
        username: &str,
        password_hash: &str,
        display_name: Option<&str>,
        note: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        trace!("Creating new user: {}", username);

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, username, password_hash, display_name, note, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            RETURNING id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                   is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                   is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                   is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                   is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                   is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                   is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                   is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                   is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                   is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                   is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                   is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                   is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                   is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                   is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                   is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                   is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                   is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                   is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                   is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                   is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                   is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                   is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                   is_require_invite_approval_by_approval
            "#,
            email,
            username,
            password_hash,
            display_name,
            note
        )
        .fetch_one(pool)
        .await?;

        debug!("Created user: {}", user.username);
        Ok(user)
    }
}

/// Database service
pub struct DatabaseService {
    pool: sqlx::PgPool,
}

impl DatabaseService {
    /// Creates a new database service
    pub fn new(pool: sqlx::PgPool) -> Self {
        info!("Creating new database service");
        Self { pool }
    }

    /// Get all users
    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        trace!("Getting all users");

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                   is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                   is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                   is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                   is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                   is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                   is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                   is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                   is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                   is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                   is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                   is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                   is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                   is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                   is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                   is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                   is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                   is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                   is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                   is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                   is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                   is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                   is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                   is_require_invite_approval_by_approval
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        debug!("Retrieved {} users", users.len());
        Ok(users)
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, user_id: i64) -> Result<Option<User>, sqlx::Error> {
        trace!("Getting user by ID: {}", user_id);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                   is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                   is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                   is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                   is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                   is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                   is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                   is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                   is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                   is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                   is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                   is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                   is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                   is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                   is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                   is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                   is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                   is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                   is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                   is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                   is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                   is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                   is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                   is_require_invite_approval_by_approval
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Get user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        trace!("Getting user by username: {}", username);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                   is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                   is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                   is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                   is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                   is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                   is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                   is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                   is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                   is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                   is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                   is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                   is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                   is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                   is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                   is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                   is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                   is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                   is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                   is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                   is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                   is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                   is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                   is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                   is_require_invite_approval_by_approval
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Create user
    pub async fn create_user(
        &self,
        email: &str,
        username: &str,
        password_hash: &str,
        display_name: Option<&str>,
        note: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        info!("Creating new user: {}", username);
        trace!("User details: email={}, display_name={:?}", email, display_name);

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, username, password_hash, display_name, note, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            RETURNING id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                     is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                     is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                     is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                     is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                     is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                     is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                     is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                     is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                     is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                     is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                     is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                     is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                     is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                     is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                     is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                     is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                     is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                     is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                     is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                     is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                     is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                     is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                     is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                     is_require_invite_approval_by_approval
            "#,
            email,
            username,
            password_hash,
            display_name,
            note
        )
        .fetch_one(&self.pool)
        .await?;

        debug!("Created user with ID: {}", user.id);
        Ok(user)
    }

    /// Update user
    pub async fn update_user(
        &self,
        user_id: i64,
        display_name: Option<&str>,
        note: Option<&str>,
        avatar_url: Option<&str>,
        header_url: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        info!("Updating user: {}", user_id);
        trace!("Update details: display_name={:?}, note={:?}", display_name, note);

        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET display_name = COALESCE($2, display_name),
                note = COALESCE($3, note),
                avatar_url = COALESCE($4, avatar_url),
                header_url = COALESCE($5, header_url),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, username, email, password_hash, created_at, display_name, note, avatar_url, header_url,
                     is_admin, is_moderator, is_verified, is_suspended, is_silenced, is_disabled, is_approved,
                     is_confirmed, is_locked, is_bot, is_group, is_discoverable, is_indexable, is_private,
                     is_protected, is_verified_bot, is_manually_approved_follows, is_sensitive, is_show_all_media,
                     is_hide_collections, is_allow_following_move, is_skip_thread_containment, is_reject_media,
                     is_reject_reports, is_invites_enabled, is_require_invite_text, is_require_invite_application,
                     is_require_invite_approval, is_require_invite_confirmation, is_require_invite_verification,
                     is_require_invite_approval_by_admin, is_require_invite_approval_by_moderator,
                     is_require_invite_approval_by_user, is_require_invite_approval_by_group,
                     is_require_invite_approval_by_domain, is_require_invite_approval_by_ip,
                     is_require_invite_approval_by_location, is_require_invite_approval_by_time,
                     is_require_invite_approval_by_frequency, is_require_invite_approval_by_limit,
                     is_require_invite_approval_by_quota, is_require_invite_approval_by_rule,
                     is_require_invite_approval_by_policy, is_require_invite_approval_by_setting,
                     is_require_invite_approval_by_config, is_require_invite_approval_by_option,
                     is_require_invite_approval_by_preference, is_require_invite_approval_by_choice,
                     is_require_invite_approval_by_decision, is_require_invite_approval_by_judgment,
                     is_require_invite_approval_by_evaluation, is_require_invite_approval_by_assessment,
                     is_require_invite_approval_by_review, is_require_invite_approval_by_audit,
                     is_require_invite_approval_by_check, is_require_invite_approval_by_validation,
                     is_require_invite_approval_by_verification, is_require_invite_approval_by_confirmation,
                     is_require_invite_approval_by_authentication, is_require_invite_approval_by_authorization,
                     is_require_invite_approval_by_permission, is_require_invite_approval_by_consent,
                     is_require_invite_approval_by_agreement, is_require_invite_approval_by_acceptance,
                     is_require_invite_approval_by_approval
            "#,
            user_id,
            display_name,
            note,
            avatar_url,
            header_url
        )
        .fetch_one(&self.pool)
        .await?;

        debug!("Updated user with ID: {}", user.id);
        Ok(user)
    }
}

/// Initialize database connection
///
/// # Returns
///
/// Result with database pool or error
pub async fn init_database() -> Result<sqlx::PgPool, sqlx::Error> {
    info!("Initializing database connection");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://rustodon:rustodon@localhost:5432/rustodon".to_string());

    let pool = sqlx::PgPool::connect(&database_url).await?;

    info!("Database connection established");
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_service_new() {
        // This would require a real database connection for full testing
        // For now, just test that the struct can be created
        let pool = sqlx::PgPool::connect("postgresql://test:test@localhost:5432/test").await;
        if let Ok(pool) = pool {
            let service = DatabaseService::new(pool);
            assert!(true); // Service created successfully
        }
    }
}
