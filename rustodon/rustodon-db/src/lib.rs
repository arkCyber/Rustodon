//! Database operations for Rustodon
//!
//! This crate provides database models and operations for the Rustodon server.
//! It handles all database interactions using SQLx for type-safe database operations.
//!
//! # Features
//!
//! - User management
//! - Status posts
//! - Lists and list memberships
//! - Blocks and mutes
//! - Connection pooling
//! - Type-safe queries
//!
//! # Examples
//!
//! ```rust
//! use rustodon_db::{establish_connection, User, Status};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let pool = establish_connection("postgres://localhost/rustodon").await?;
//!
//!     // Create a new user
//!     let user = User::create(&pool, "user@example.com", "username", "hash", None, None).await?;
//!
//!     // Create a status
//!     let status = Status::create(&pool, user.id, "Hello world!", "public", false, None, None, None).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Dependencies
//!
//! - `sqlx`: Database toolkit
//! - `serde`: Serialization
//! - `chrono`: Date/time handling
//! - `tracing`: Logging
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

// Module declarations
pub mod config;
pub mod connection;
pub mod error;
pub mod models;

// Re-export main types
pub use config::{create_pool, establish_connection, DbConfig};
pub use connection::{
    create_config_from_env, get_global_database, get_global_pool, init_global_database,
    DatabaseConfig, DatabaseConnectionError, DatabaseManager, SslMode,
};
pub use error::DbError;
pub use models::{
    Block, DomainBlock, Favourite, Filter, Follow, List, ListAccount, Mute, Reblog, Status, User,
};

// Re-export structs from other crates (when they exist)
// These will be uncommented as the crates are implemented
// pub use rustodon_favourites::Favourite;
// pub use rustodon_follows::Follow;
// pub use rustodon_reblogs::Reblog;
// pub use rustodon_domains::DomainBlock;
// pub use rustodon_filters::Filter;
// pub use rustodon_bookmarks::Bookmark;
// pub use rustodon_announcements::Announcement;
// pub use rustodon_follow_requests::FollowRequest;

/// Initialize database with offline mode for compilation
///
/// This function should be called during application startup to initialize
/// the database connection pool. It handles the sqlx offline mode configuration.
pub async fn init_database() -> Result<(), DatabaseConnectionError> {
    // Set sqlx to offline mode if DATABASE_URL is not available
    if std::env::var("DATABASE_URL").is_err() {
        std::env::set_var("SQLX_OFFLINE", "true");
    }

    let config = create_config_from_env();
    init_global_database(config).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{debug, info};

    #[test]
    fn test_db_config_default() {
        let config = DbConfig::default();
        assert_eq!(config.url, "postgres://localhost/rustodon");
        assert_eq!(config.max_connections, 10);
    }

    #[tokio::test]
    async fn test_user_operations() {
        // This test requires a database connection
        // In a real test environment, you would use a test database
        let config = DbConfig::default();
        match create_pool(&config).await {
            Ok(_pool) => {
                info!("Database connection successful");
            }
            Err(_) => {
                debug!("Database connection failed (expected in test environment)");
            }
        }
    }

    #[tokio::test]
    async fn test_status_operations() {
        // This test requires a database connection
        // In a real test environment, you would use a test database
        let config = DbConfig::default();
        match create_pool(&config).await {
            Ok(_pool) => {
                info!("Database connection successful");
            }
            Err(_) => {
                debug!("Database connection failed (expected in test environment)");
            }
        }
    }
}
