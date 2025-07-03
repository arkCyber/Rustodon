#!/bin/bash

# Database setup script for Rustodon
# This script creates the database and runs initial migrations

set -e

echo "Setting up Rustodon database..."

# Database configuration
DB_NAME="rustodon"
DB_USER="postgres"
DB_HOST="localhost"
DB_PORT="5432"

# Check if PostgreSQL is running
if ! pg_isready -h $DB_HOST -p $DB_PORT -U $DB_USER; then
    echo "Error: PostgreSQL is not running on $DB_HOST:$DB_PORT"
    echo "Please start PostgreSQL and try again"
    exit 1
fi

# Create database if it doesn't exist
echo "Creating database '$DB_NAME' if it doesn't exist..."
createdb -h $DB_HOST -p $DB_PORT -U $DB_USER $DB_NAME 2>/dev/null || echo "Database '$DB_NAME' already exists"

# Create basic tables
echo "Creating basic tables..."
psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME << 'EOF'

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    note TEXT,
    avatar_url VARCHAR(255),
    header_url VARCHAR(255),
    locked BOOLEAN DEFAULT FALSE,
    bot BOOLEAN DEFAULT FALSE,
    discoverable BOOLEAN DEFAULT TRUE,
    group_account BOOLEAN DEFAULT FALSE,
    last_status_at TIMESTAMP,
    statuses_count BIGINT DEFAULT 0,
    followers_count BIGINT DEFAULT 0,
    following_count BIGINT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Statuses table
CREATE TABLE IF NOT EXISTS statuses (
    id BIGSERIAL PRIMARY KEY,
    uri VARCHAR(255) NOT NULL UNIQUE,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    visibility VARCHAR(50) NOT NULL DEFAULT 'public',
    sensitive BOOLEAN DEFAULT FALSE,
    spoiler_text VARCHAR(255),
    in_reply_to_id BIGINT REFERENCES statuses(id) ON DELETE SET NULL,
    in_reply_to_account_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    reblog_of_id BIGINT REFERENCES statuses(id) ON DELETE SET NULL,
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

-- Lists table
CREATE TABLE IF NOT EXISTS lists (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- List accounts (many-to-many relationship)
CREATE TABLE IF NOT EXISTS list_accounts (
    list_id BIGINT NOT NULL REFERENCES lists(id) ON DELETE CASCADE,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (list_id, account_id)
);

-- Blocks table
CREATE TABLE IF NOT EXISTS blocks (
    id BIGSERIAL PRIMARY KEY,
    blocker_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    blocked_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(blocker_id, blocked_id)
);

-- Mutes table
CREATE TABLE IF NOT EXISTS mutes (
    id BIGSERIAL PRIMARY KEY,
    muter_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    muted_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    hide_notifications BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(muter_id, muted_id)
);

-- Follows table
CREATE TABLE IF NOT EXISTS follows (
    id BIGSERIAL PRIMARY KEY,
    follower_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    followed_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    show_reblogs BOOLEAN DEFAULT TRUE,
    notify BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(follower_id, followed_id)
);

-- Favourites table
CREATE TABLE IF NOT EXISTS favourites (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status_id BIGINT NOT NULL REFERENCES statuses(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id, status_id)
);

-- Reblogs table
CREATE TABLE IF NOT EXISTS reblogs (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status_id BIGINT NOT NULL REFERENCES statuses(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id, status_id)
);

-- Bookmarks table
CREATE TABLE IF NOT EXISTS bookmarks (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status_id BIGINT NOT NULL REFERENCES statuses(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id, status_id)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_statuses_account_id ON statuses(account_id);
CREATE INDEX IF NOT EXISTS idx_statuses_created_at ON statuses(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_lists_account_id ON lists(account_id);
CREATE INDEX IF NOT EXISTS idx_blocks_blocker_id ON blocks(blocker_id);
CREATE INDEX IF NOT EXISTS idx_blocks_blocked_id ON blocks(blocked_id);
CREATE INDEX IF NOT EXISTS idx_mutes_muter_id ON mutes(muter_id);
CREATE INDEX IF NOT EXISTS idx_mutes_muted_id ON mutes(muted_id);
CREATE INDEX IF NOT EXISTS idx_follows_follower_id ON follows(follower_id);
CREATE INDEX IF NOT EXISTS idx_follows_followed_id ON follows(followed_id);
CREATE INDEX IF NOT EXISTS idx_favourites_account_id ON favourites(account_id);
CREATE INDEX IF NOT EXISTS idx_favourites_status_id ON favourites(status_id);
CREATE INDEX IF NOT EXISTS idx_reblogs_account_id ON reblogs(account_id);
CREATE INDEX IF NOT EXISTS idx_reblogs_status_id ON reblogs(status_id);
CREATE INDEX IF NOT EXISTS idx_bookmarks_account_id ON bookmarks(account_id);
CREATE INDEX IF NOT EXISTS idx_bookmarks_status_id ON bookmarks(status_id);

EOF

echo "Database setup completed successfully!"
echo "You can now run: DATABASE_URL='postgres://postgres@localhost/rustodon' cargo check"
