-- Migration: Create statuses table
-- Author: arkSong (arksong2018@gmail.com)
-- Description: Creates the statuses table with visibility, content, and relationship fields

-- Create status_visibility enum type
DO $$ BEGIN
    CREATE TYPE status_visibility AS ENUM ('public', 'unlisted', 'private', 'direct');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Create status_type enum type
DO $$ BEGIN
    CREATE TYPE status_type AS ENUM ('status', 'reblog', 'reply');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Create statuses table
CREATE TABLE IF NOT EXISTS statuses (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    visibility status_visibility NOT NULL DEFAULT 'public',
    sensitive BOOLEAN NOT NULL DEFAULT false,
    spoiler_text TEXT,
    in_reply_to_id BIGINT REFERENCES statuses(id) ON DELETE SET NULL,
    in_reply_to_account_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    reblog_of_id BIGINT REFERENCES statuses(id) ON DELETE SET NULL,
    status_type status_type NOT NULL DEFAULT 'status',
    language VARCHAR(10),
    url VARCHAR(255),
    uri VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP,
    local BOOLEAN NOT NULL DEFAULT true,
    federated BOOLEAN NOT NULL DEFAULT true,
    favourites_count INTEGER NOT NULL DEFAULT 0,
    reblogs_count INTEGER NOT NULL DEFAULT 0,
    replies_count INTEGER NOT NULL DEFAULT 0,
    media_attachments JSONB,
    mentions JSONB,
    tags JSONB,
    emojis JSONB,
    poll JSONB,
    application JSONB
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_statuses_account_id ON statuses(account_id);
CREATE INDEX IF NOT EXISTS idx_statuses_visibility ON statuses(visibility);
CREATE INDEX IF NOT EXISTS idx_statuses_created_at ON statuses(created_at);
CREATE INDEX IF NOT EXISTS idx_statuses_in_reply_to_id ON statuses(in_reply_to_id);
CREATE INDEX IF NOT EXISTS idx_statuses_reblog_of_id ON statuses(reblog_of_id);
CREATE INDEX IF NOT EXISTS idx_statuses_deleted_at ON statuses(deleted_at);
CREATE INDEX IF NOT EXISTS idx_statuses_local ON statuses(local);
CREATE INDEX IF NOT EXISTS idx_statuses_federated ON statuses(federated);

-- Create trigger to update updated_at timestamp
CREATE TRIGGER update_statuses_updated_at
    BEFORE UPDATE ON statuses
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Create trigger to update user's last_status_at
CREATE OR REPLACE FUNCTION update_user_last_status_at()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE users
        SET last_status_at = NEW.created_at
        WHERE id = NEW.account_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE users
        SET last_status_at = (
            SELECT MAX(created_at)
            FROM statuses
            WHERE account_id = OLD.account_id AND deleted_at IS NULL
        )
        WHERE id = OLD.account_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_user_last_status_at_trigger
    AFTER INSERT OR DELETE ON statuses
    FOR EACH ROW
    EXECUTE FUNCTION update_user_last_status_at();
