-- Migration: Create follows table
-- Author: arkSong (arksong2018@gmail.com)
-- Description: Creates the follows table with follow relationships, requests, and preferences

-- Create follows table
CREATE TABLE follows (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    active BOOLEAN NOT NULL DEFAULT true,
    pending BOOLEAN NOT NULL DEFAULT false,
    muted BOOLEAN NOT NULL DEFAULT false,
    blocked BOOLEAN NOT NULL DEFAULT false,
    show_reblogs BOOLEAN NOT NULL DEFAULT true,
    notify BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP,
    UNIQUE(account_id, target_account_id)
);

-- Create indexes for performance
CREATE INDEX idx_follows_account_id ON follows(account_id);
CREATE INDEX idx_follows_target_account_id ON follows(target_account_id);
CREATE INDEX idx_follows_active ON follows(active);
CREATE INDEX idx_follows_pending ON follows(pending);
CREATE INDEX idx_follows_created_at ON follows(created_at);
CREATE INDEX idx_follows_deleted_at ON follows(deleted_at);

-- Create trigger to update updated_at timestamp
CREATE TRIGGER update_follows_updated_at
    BEFORE UPDATE ON follows
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Create trigger to update user follower/following counts
CREATE OR REPLACE FUNCTION update_user_follow_counts()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- Update following count for follower
        UPDATE users
        SET following_count = following_count + 1
        WHERE id = NEW.account_id;

        -- Update followers count for target
        UPDATE users
        SET followers_count = followers_count + 1
        WHERE id = NEW.target_account_id;

        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        -- Update following count for follower
        UPDATE users
        SET following_count = GREATEST(following_count - 1, 0)
        WHERE id = OLD.account_id;

        -- Update followers count for target
        UPDATE users
        SET followers_count = GREATEST(followers_count - 1, 0)
        WHERE id = OLD.target_account_id;

        RETURN OLD;
    ELSIF TG_OP = 'UPDATE' THEN
        -- Handle status changes
        IF OLD.active = false AND NEW.active = true THEN
            -- Follow was accepted
            UPDATE users
            SET following_count = following_count + 1
            WHERE id = NEW.account_id;

            UPDATE users
            SET followers_count = followers_count + 1
            WHERE id = NEW.target_account_id;
        ELSIF OLD.active = true AND NEW.active = false THEN
            -- Follow was removed
            UPDATE users
            SET following_count = GREATEST(following_count - 1, 0)
            WHERE id = NEW.account_id;

            UPDATE users
            SET followers_count = GREATEST(followers_count - 1, 0)
            WHERE id = NEW.target_account_id;
        END IF;

        RETURN NEW;
    END IF;
    RETURN NULL;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_user_follow_counts_trigger
    AFTER INSERT OR DELETE OR UPDATE ON follows
    FOR EACH ROW
    EXECUTE FUNCTION update_user_follow_counts();
