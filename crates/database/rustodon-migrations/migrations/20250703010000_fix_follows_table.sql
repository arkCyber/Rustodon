-- Migration: Fix follows table structure
-- Author: arkSong (arksong2018@gmail.com)
-- Description: Ensures follows table has all required columns for user relationships

ALTER TABLE follows
    ADD COLUMN IF NOT EXISTS follower_id BIGINT NOT NULL,
    ADD COLUMN IF NOT EXISTS followed_id BIGINT NOT NULL,
    ADD COLUMN IF NOT EXISTS show_reblogs BOOLEAN NOT NULL DEFAULT TRUE,
    ADD COLUMN IF NOT EXISTS notify BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP NOT NULL DEFAULT NOW();

CREATE INDEX IF NOT EXISTS idx_follows_follower_id ON follows(follower_id);
CREATE INDEX IF NOT EXISTS idx_follows_followed_id ON follows(followed_id);
