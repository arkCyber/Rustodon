-- Migration: Add user_status enum and status column
-- Author: arkSong (arksong2018@gmail.com)
-- Description: Ensures the user_status enum type exists and adds the status column to users table

-- This migration is now a no-op because user_status type and status column are created in 20250101000001_create_users_table.sql
-- If you need to patch or update the enum or column, add ALTER statements below.

-- Example: Add comment for status column (safe to run multiple times)
COMMENT ON COLUMN users.status IS 'User account status (active, suspended, deleted, unconfirmed)';
