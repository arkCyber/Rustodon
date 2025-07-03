//!
//! Rustodon Migrations CLI
//!
//! Provides command-line interface for running, creating, and resetting database migrations.
//!
//! # Usage
//!
//! ```sh
//! cargo run -p rustodon-migrations -- migrate
//! cargo run -p rustodon-migrations -- reset
//! cargo run -p rustodon-migrations -- create migration_name
//! ```
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use clap::{Parser, Subcommand};
use rustodon_migrations::MigrationError;
use sqlx::migrate::Migrator;
use std::path::Path;
use tracing::info;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations_clean");

#[derive(Parser, Debug)]
#[command(name = "rustodon-migrations")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run all pending migrations
    Migrate,
    /// Reset the database (drop and recreate)
    Reset,
    /// Create a new migration
    Create {
        /// Name of the migration
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), MigrationError> {
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_target(false)
        .init();
    let cli = Cli::parse();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match cli.command {
        Commands::Migrate => {
            info!("Running migrations...");
            let pool = sqlx::PgPool::connect(&database_url).await?;
            MIGRATOR.run(&pool).await?;
            info!("Migrations complete.");
        }
        Commands::Reset => {
            info!("Resetting database...");
            let pool = sqlx::PgPool::connect(&database_url).await?;
            sqlx::query("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")
                .execute(&pool)
                .await?;
            MIGRATOR.run(&pool).await?;
            info!("Database reset and migrated.");
        }
        Commands::Create { name } => {
            info!("Creating new migration: {}", name);
            let migrations_dir = Path::new("./migrations");
            if !migrations_dir.exists() {
                std::fs::create_dir_all(migrations_dir)?;
            }
            let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
            let up = migrations_dir.join(format!("{}_{}_up.sql", timestamp, name));
            let down = migrations_dir.join(format!("{}_{}_down.sql", timestamp, name));
            std::fs::write(&up, b"-- Up migration\n")?;
            std::fs::write(&down, b"-- Down migration\n")?;
            info!("Created migration files: {:?}, {:?}", up, down);
        }
    }
    Ok(())
}
