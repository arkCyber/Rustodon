//!
//! Common test utilities for Rustodon integration tests
//!
//! Provides database setup, test helpers, and shared infrastructure for end-to-end testing.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use sqlx::PgPool;
use tracing::{info, debug};

/// Test database configuration
pub struct TestDb {
    pub pool: PgPool,
}

impl TestDb {
    /// Create a new test database connection
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/rustodon_test".to_string());

        info!("Connecting to test database: {}", database_url);
        let pool = PgPool::connect(&database_url).await?;

        // Run migrations
        debug!("Running test database migrations");
        sqlx::migrate!("../rustodon-migrations/migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    /// Clean up test data
    pub async fn cleanup(&self) -> Result<(), sqlx::Error> {
        debug!("Cleaning up test database");
        sqlx::query("DELETE FROM users").execute(&self.pool).await?;
        Ok(())
    }
}

/// Test user data for integration tests
pub struct TestUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Default for TestUser {
    fn default() -> Self {
        Self {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "testpassword123".to_string(),
        }
    }
}

/// Initialize test logging
pub fn init_test_logging() {
    let _ = tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_target(false)
        .try_init();
}
