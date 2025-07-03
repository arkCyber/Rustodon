-- Migration: Create users table
-- Author: arkSong (arksong2018@gmail.com)
-- Description: Creates the users table with authentication and profile fields

-- Create user_status enum type if it doesn't exist
DO $$ BEGIN
    CREATE TYPE user_status AS ENUM ('active', 'suspended', 'deleted', 'unconfirmed');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    note TEXT,
    status user_status NOT NULL DEFAULT 'unconfirmed',
    locked BOOLEAN NOT NULL DEFAULT false,
    bot BOOLEAN NOT NULL DEFAULT false,
    discoverable BOOLEAN NOT NULL DEFAULT true,
    group_account BOOLEAN NOT NULL DEFAULT false,
    avatar VARCHAR(255),
    header VARCHAR(255),
    website VARCHAR(255),
    location VARCHAR(255),
    language VARCHAR(10),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_active_at TIMESTAMP,
    confirmation_token VARCHAR(255),
    confirmed_at TIMESTAMP,
    recovery_email VARCHAR(255),
    statuses_count INTEGER NOT NULL DEFAULT 0,
    followers_count INTEGER NOT NULL DEFAULT 0,
    following_count INTEGER NOT NULL DEFAULT 0,
    last_status_at TIMESTAMP
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_status ON users(status);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);
CREATE INDEX IF NOT EXISTS idx_users_confirmation_token ON users(confirmation_token);

-- Create trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
