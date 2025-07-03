-- Migration: Add missing columns to users table
-- Author: arkSong (arksong2018@gmail.com)
-- Description: Adds missing columns that are referenced in the User model but not in the database schema

-- Add missing columns to users table
ALTER TABLE users
ADD COLUMN IF NOT EXISTS reset_password_token VARCHAR(255),
ADD COLUMN IF NOT EXISTS reset_password_sent_at TIMESTAMP,
ADD COLUMN IF NOT EXISTS remember_created_at TIMESTAMP,
ADD COLUMN IF NOT EXISTS sign_in_count INTEGER NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS current_sign_in_at TIMESTAMP,
ADD COLUMN IF NOT EXISTS last_sign_in_at TIMESTAMP,
ADD COLUMN IF NOT EXISTS current_sign_in_ip INET,
ADD COLUMN IF NOT EXISTS last_sign_in_ip INET,
ADD COLUMN IF NOT EXISTS admin BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN IF NOT EXISTS moderator BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN IF NOT EXISTS approved BOOLEAN NOT NULL DEFAULT true;

-- Create indexes for new columns
CREATE INDEX IF NOT EXISTS idx_users_reset_password_token ON users(reset_password_token);
CREATE INDEX IF NOT EXISTS idx_users_admin ON users(admin);
CREATE INDEX IF NOT EXISTS idx_users_moderator ON users(moderator);
CREATE INDEX IF NOT EXISTS idx_users_approved ON users(approved);

-- Add comments for new columns
COMMENT ON COLUMN users.reset_password_token IS 'Token for password reset functionality';
COMMENT ON COLUMN users.reset_password_sent_at IS 'When password reset token was sent';
COMMENT ON COLUMN users.remember_created_at IS 'When remember me was created';
COMMENT ON COLUMN users.sign_in_count IS 'Number of successful sign-ins';
COMMENT ON COLUMN users.current_sign_in_at IS 'Current sign-in timestamp';
COMMENT ON COLUMN users.last_sign_in_at IS 'Last sign-in timestamp';
COMMENT ON COLUMN users.current_sign_in_ip IS 'Current sign-in IP address';
COMMENT ON COLUMN users.last_sign_in_ip IS 'Last sign-in IP address';
COMMENT ON COLUMN users.admin IS 'Whether user has admin privileges';
COMMENT ON COLUMN users.moderator IS 'Whether user has moderator privileges';
COMMENT ON COLUMN users.approved IS 'Whether user account is approved';
