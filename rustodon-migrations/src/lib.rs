//!
//! Rustodon Migrations Library
//!
//! Provides migration helpers and error types for the Rustodon database schema.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use sqlx::postgres::PgPool;
use thiserror::Error;
use tracing::{error, info};

/// Error type for migration operations
#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Migration error: {0}")]
    Migration(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Migrate error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Create favourites table migration
pub async fn create_favourites_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("Creating favourites table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS favourites (
            id BIGSERIAL PRIMARY KEY,
            user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            status_id VARCHAR(255) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            UNIQUE(user_id, status_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    info!("Favourites table created successfully");
    Ok(())
}

/// Run all migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("Running database migrations");

    create_favourites_table(pool).await?;

    info!("All migrations completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_error_display() {
        let err = MigrationError::Migration("fail".to_string());
        assert_eq!(format!("{}", err), "Migration error: fail");
    }
}
