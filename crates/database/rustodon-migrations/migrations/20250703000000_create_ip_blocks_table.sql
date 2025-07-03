-- Migration: Create ip_blocks table
-- Author: arkSong (arksong2018@gmail.com)
-- Description: Creates the ip_blocks table for IP-based blocking and moderation

CREATE TABLE IF NOT EXISTS ip_blocks (
    id BIGSERIAL PRIMARY KEY,
    ip_address INET NOT NULL,
    cidr_range INTEGER,
    severity VARCHAR(32) NOT NULL DEFAULT 'none',
    reason TEXT,
    expires_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(ip_address, cidr_range)
);

CREATE INDEX IF NOT EXISTS idx_ip_blocks_ip_address ON ip_blocks(ip_address);
CREATE INDEX IF NOT EXISTS idx_ip_blocks_severity ON ip_blocks(severity);
CREATE INDEX IF NOT EXISTS idx_ip_blocks_expires_at ON ip_blocks(expires_at);

-- Trigger to update updated_at
CREATE TRIGGER update_ip_blocks_updated_at
    BEFORE UPDATE ON ip_blocks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
