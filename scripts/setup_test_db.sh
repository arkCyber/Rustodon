#!/bin/bash

# Rustodon Test Database Setup Script
# This script sets up a PostgreSQL test database for Rustodon development
# Author: arkSong (arksong2018@gmail.com)

set -e

echo "Setting up Rustodon test database..."

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "PostgreSQL is not installed. Please install PostgreSQL first."
    echo "On macOS: brew install postgresql"
    echo "On Ubuntu: sudo apt-get install postgresql postgresql-contrib"
    exit 1
fi

# Check if PostgreSQL service is running
if ! pg_isready -q; then
    echo "PostgreSQL service is not running. Starting PostgreSQL..."
    if command -v brew &> /dev/null; then
        brew services start postgresql
    elif command -v systemctl &> /dev/null; then
        sudo systemctl start postgresql
    else
        echo "Please start PostgreSQL service manually"
        exit 1
    fi
fi

# Create test database
echo "Creating test database..."
createdb rustodon_test 2>/dev/null || echo "Database rustodon_test already exists"

# Create test user if it doesn't exist
echo "Setting up test user..."
psql -d rustodon_test -c "CREATE USER IF NOT EXISTS rustodon_test WITH PASSWORD 'test_password';" 2>/dev/null || true
psql -d rustodon_test -c "GRANT ALL PRIVILEGES ON DATABASE rustodon_test TO rustodon_test;" 2>/dev/null || true

# Create tables
echo "Creating database tables..."
psql -d rustodon_test -c "
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    note TEXT,
    avatar_url VARCHAR(500),
    header_url VARCHAR(500),
    locked BOOLEAN DEFAULT FALSE,
    bot BOOLEAN DEFAULT FALSE,
    discoverable BOOLEAN DEFAULT TRUE,
    group_account BOOLEAN DEFAULT FALSE,
    last_status_at TIMESTAMP,
    statuses_count BIGINT DEFAULT 0,
    followers_count BIGINT DEFAULT 0,
    following_count BIGINT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS statuses (
    id SERIAL PRIMARY KEY,
    uri VARCHAR(500) UNIQUE NOT NULL,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    visibility VARCHAR(50) NOT NULL DEFAULT 'public',
    sensitive BOOLEAN DEFAULT FALSE,
    spoiler_text VARCHAR(500),
    in_reply_to_id BIGINT REFERENCES statuses(id) ON DELETE CASCADE,
    in_reply_to_account_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    reblog_of_id BIGINT REFERENCES statuses(id) ON DELETE CASCADE,
    application_id BIGINT,
    language VARCHAR(10),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    replies_count BIGINT DEFAULT 0,
    reblogs_count BIGINT DEFAULT 0,
    favourites_count BIGINT DEFAULT 0,
    reblog BOOLEAN DEFAULT FALSE,
    reply BOOLEAN DEFAULT FALSE,
    direct BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS follows (
    id SERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id, target_account_id)
);

CREATE TABLE IF NOT EXISTS blocks (
    id SERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id, target_account_id)
);

CREATE TABLE IF NOT EXISTS mutes (
    id SERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id, target_account_id)
);

CREATE TABLE IF NOT EXISTS favourites (
    id SERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status_id BIGINT NOT NULL REFERENCES statuses(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id, status_id)
);

CREATE TABLE IF NOT EXISTS reblogs (
    id SERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status_id BIGINT NOT NULL REFERENCES statuses(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id, status_id)
);

CREATE TABLE IF NOT EXISTS lists (
    id SERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    replies_policy VARCHAR(50) DEFAULT 'list',
    exclusive BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS list_accounts (
    id SERIAL PRIMARY KEY,
    list_id BIGINT NOT NULL REFERENCES lists(id) ON DELETE CASCADE,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(list_id, account_id)
);

CREATE TABLE IF NOT EXISTS domain_blocks (
    id SERIAL PRIMARY KEY,
    domain VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS filters (
    id SERIAL PRIMARY KEY,
    phrase VARCHAR(255) NOT NULL,
    context TEXT[] NOT NULL,
    expires_at TIMESTAMP,
    irreversible BOOLEAN DEFAULT FALSE,
    whole_word BOOLEAN DEFAULT FALSE,
    action VARCHAR(50) DEFAULT 'hide',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
"

echo "Test database setup complete!"
echo "You can now run tests with: DATABASE_URL=postgres://rustodon_test:test_password@localhost/rustodon_test cargo test"
