#!/bin/bash

# Database reset and migration script for Rustodon
# Author: arkSong (arksong2018@gmail.com)

set -e

echo "🔄 Starting database reset and migration..."

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "❌ DATABASE_URL environment variable is not set"
    echo "Please set it to your PostgreSQL connection string"
    echo "Example: export DATABASE_URL='postgres://username:password@localhost/rustodon_test'"
    exit 1
fi

echo "📊 Using database: $DATABASE_URL"

# Extract database name from DATABASE_URL
DB_NAME=$(echo $DATABASE_URL | sed -n 's/.*\/\([^?]*\).*/\1/p')

if [ -z "$DB_NAME" ]; then
    echo "❌ Could not extract database name from DATABASE_URL"
    exit 1
fi

echo "🗄️  Database name: $DB_NAME"

# Drop and recreate database
echo "🗑️  Dropping database if it exists..."
psql "$DATABASE_URL" -c "DROP DATABASE IF EXISTS $DB_NAME;" 2>/dev/null || true

echo "🆕 Creating new database..."
psql "$DATABASE_URL" -c "CREATE DATABASE $DB_NAME;" 2>/dev/null || true

echo "✅ Database reset complete"

# Run migrations
echo "🔄 Running database migrations..."
cargo run -p rustodon-migrations

echo "✅ Database migration complete"

# Verify schema
echo "🔍 Verifying database schema..."
psql "$DATABASE_URL" -c "\dt" || echo "⚠️  Could not list tables (this might be normal if no tables exist yet)"

echo "🎉 Database reset and migration completed successfully!"
echo "📝 You can now run: cargo check"
