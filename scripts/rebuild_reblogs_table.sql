-- SQL Script to Rebuild Reblogs Table
-- Author: arkSong (arksong2018@gmail.com)
-- Description: Rebuilds the reblogs table with the latest schema
-- Usage: psql -d your_database -f rebuild_reblogs_table.sql

-- Drop existing table and indexes if they exist
DROP TABLE IF EXISTS reblogs CASCADE;

-- Create reblogs table with latest schema
CREATE TABLE reblogs (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    account_id BIGINT NOT NULL,
    status_id BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
    FOREIGN KEY (account_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (status_id) REFERENCES statuses(id) ON DELETE CASCADE,
    UNIQUE(account_id, status_id)
);

-- Create indexes for better performance
CREATE INDEX idx_reblogs_account_id ON reblogs(account_id);
CREATE INDEX idx_reblogs_status_id ON reblogs(status_id);
CREATE INDEX idx_reblogs_created_at ON reblogs(created_at DESC);

-- Add comments for documentation
COMMENT ON TABLE reblogs IS 'Stores user reblogs (reposts) of statuses';
COMMENT ON COLUMN reblogs.id IS 'Unique identifier for the reblog';
COMMENT ON COLUMN reblogs.account_id IS 'ID of the user who reblogged the status';
COMMENT ON COLUMN reblogs.status_id IS 'ID of the status that was reblogged';
COMMENT ON COLUMN reblogs.created_at IS 'Timestamp when the reblog was created';

-- Verify table creation
SELECT 'Reblogs table created successfully' as status;
SELECT COUNT(*) as table_count FROM information_schema.tables WHERE table_name = 'reblogs';
