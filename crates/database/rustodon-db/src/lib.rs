//!
//! Rustodon Database Operations
//!
//! Provides database operations for the Rustodon server, including user management,
//! status operations, and other database-related functionality.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use chrono::NaiveDateTime;
use sqlx::PgPool;
use tracing::{debug, info, trace};

/// User model - simplified for testing
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub note: Option<String>,
    pub locked: bool,
    pub bot: bool,
    pub discoverable: bool,
    pub group_account: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    /// Get user by ID
    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        trace!("Getting user by ID: {}", id);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, note, locked, bot,
                   discoverable, group_account, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(ref user) = user {
            debug!("Found user: {}", user.username);
        } else {
            debug!("User not found for ID: {}", id);
        }

        Ok(user)
    }

    /// Get user by username
    pub async fn get_by_username(
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        trace!("Getting user by username: {}", username);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, note, locked, bot,
                   discoverable, group_account, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        if let Some(ref user) = user {
            debug!("Found user: {}", user.username);
        } else {
            debug!("User not found for username: {}", username);
        }

        Ok(user)
    }

    /// Get user by email
    pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        trace!("Getting user by email: {}", email);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, note, locked, bot,
                   discoverable, group_account, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        if let Some(ref user) = user {
            debug!("Found user: {}", user.username);
        } else {
            debug!("User not found for email: {}", email);
        }

        Ok(user)
    }

    /// Create a new user
    pub async fn create(
        pool: &PgPool,
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
            INSERT INTO users (email, username, password_hash, display_name, note, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            RETURNING id, username, email, password_hash, display_name, note, locked, bot,
                      discoverable, group_account, created_at, updated_at
            "#,
            email,
            username,
            password_hash,
            display_name,
            note
        )
        .fetch_one(pool)
        .await?;

        info!("Created new user: {} (ID: {})", user.username, user.id);
        Ok(user)
    }

    /// Get all users
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        trace!("Getting all users");

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, note, locked, bot,
                   discoverable, group_account, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        debug!("Found {} users", users.len());
        Ok(users)
    }
}

/// Database service
pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    /// Creates a new database service
    pub fn new(pool: PgPool) -> Self {
        info!("Creating new database service");
        Self { pool }
    }

    /// Get all users
    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        trace!("Getting all users");

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, note, locked, bot,
                   discoverable, group_account, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, user_id: i64) -> Result<Option<User>, sqlx::Error> {
        trace!("Getting user by ID: {}", user_id);
        User::get_by_id(&self.pool, user_id).await
    }

    /// Get user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        trace!("Getting user by username: {}", username);
        User::get_by_username(&self.pool, username).await
    }

    /// Create a new user
    pub async fn create_user(
        &self,
        email: &str,
        username: &str,
        password_hash: &str,
        display_name: Option<&str>,
        note: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        trace!("Creating new user: {}", username);
        User::create(
            &self.pool,
            email,
            username,
            password_hash,
            display_name,
            note,
        )
        .await
    }
}

/// Initialize database connection
pub async fn init_database() -> Result<PgPool, sqlx::Error> {
    info!("Initializing database connection");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    info!("Database connection established");
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_service_new() {
        let pool = init_database().await.unwrap();
        let service = DatabaseService::new(pool);
        assert!(service.pool.size() > 0);
    }
}
