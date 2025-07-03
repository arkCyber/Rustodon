-- Migration: Add user_status enum and status column
-- Author: arkSong (arksong2018@gmail.com)
-- Description: Ensures the user_status enum type exists and adds the status column to users table

-- Create user_status enum type if it doesn't exist
DO $$ BEGIN
    CREATE TYPE user_status AS ENUM ('active', 'suspended', 'deleted', 'unconfirmed');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Add status column to users table if it doesn't exist
ALTER TABLE users
ADD COLUMN IF NOT EXISTS status user_status NOT NULL DEFAULT 'unconfirmed';

-- Create index for status column if it doesn't exist
CREATE INDEX IF NOT EXISTS idx_users_status ON users(status);

-- Add comment for status column
COMMENT ON COLUMN users.status IS 'User account status (active, suspended, deleted, unconfirmed)';
