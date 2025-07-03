-- Database setup for Rustodon
-- This script creates the necessary tables for the Rustodon server

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    display_name VARCHAR(255),
    note TEXT,
    avatar_url TEXT,
    header_url TEXT,
    is_admin BOOLEAN DEFAULT FALSE,
    is_moderator BOOLEAN DEFAULT FALSE,
    is_verified BOOLEAN DEFAULT FALSE,
    is_suspended BOOLEAN DEFAULT FALSE,
    is_silenced BOOLEAN DEFAULT FALSE,
    is_disabled BOOLEAN DEFAULT FALSE,
    is_approved BOOLEAN DEFAULT FALSE,
    is_confirmed BOOLEAN DEFAULT FALSE,
    is_locked BOOLEAN DEFAULT FALSE,
    is_bot BOOLEAN DEFAULT FALSE,
    is_group BOOLEAN DEFAULT FALSE,
    is_discoverable BOOLEAN DEFAULT TRUE,
    is_indexable BOOLEAN DEFAULT TRUE,
    is_private BOOLEAN DEFAULT FALSE,
    is_protected BOOLEAN DEFAULT FALSE,
    is_verified_bot BOOLEAN DEFAULT FALSE,
    is_manually_approved_follows BOOLEAN DEFAULT FALSE,
    is_sensitive BOOLEAN DEFAULT FALSE,
    is_show_all_media BOOLEAN DEFAULT TRUE,
    is_hide_collections BOOLEAN DEFAULT FALSE,
    is_allow_following_move BOOLEAN DEFAULT TRUE,
    is_skip_thread_containment BOOLEAN DEFAULT FALSE,
    is_reject_media BOOLEAN DEFAULT FALSE,
    is_reject_reports BOOLEAN DEFAULT FALSE,
    is_invites_enabled BOOLEAN DEFAULT TRUE,
    is_require_invite_text BOOLEAN DEFAULT FALSE,
    is_require_invite_application BOOLEAN DEFAULT FALSE,
    is_require_invite_approval BOOLEAN DEFAULT FALSE,
    is_require_invite_confirmation BOOLEAN DEFAULT FALSE,
    is_require_invite_verification BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_admin BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_moderator BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_user BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_group BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_domain BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_ip BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_location BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_time BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_frequency BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_limit BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_quota BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_rule BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_policy BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_setting BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_config BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_option BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_preference BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_choice BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_decision BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_judgment BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_evaluation BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_assessment BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_review BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_audit BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_check BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_validation BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_verification BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_confirmation BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_authentication BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_authorization BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_permission BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_consent BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_agreement BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_acceptance BOOLEAN DEFAULT FALSE,
    is_require_invite_approval_by_approval BOOLEAN DEFAULT FALSE
);

-- Create follows table
CREATE TABLE IF NOT EXISTS follows (
    id BIGSERIAL PRIMARY KEY,
    follower_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    followed_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    show_reblogs BOOLEAN DEFAULT TRUE,
    notify BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(follower_id, followed_id)
);

-- Create ip_blocks table
CREATE TABLE IF NOT EXISTS ip_blocks (
    id BIGSERIAL PRIMARY KEY,
    ip_address INET NOT NULL,
    cidr_range CIDR,
    severity VARCHAR(50) NOT NULL DEFAULT 'block',
    reason TEXT NOT NULL,
    expires_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_follows_follower_id ON follows(follower_id);
CREATE INDEX IF NOT EXISTS idx_follows_followed_id ON follows(followed_id);
CREATE INDEX IF NOT EXISTS idx_ip_blocks_ip_address ON ip_blocks USING gist(ip_address inet_ops);
CREATE INDEX IF NOT EXISTS idx_ip_blocks_cidr_range ON ip_blocks USING gist(cidr_range inet_ops);
CREATE INDEX IF NOT EXISTS idx_ip_blocks_expires_at ON ip_blocks(expires_at);

-- Create other necessary tables (simplified versions)
CREATE TABLE IF NOT EXISTS statuses (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS notifications (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    from_account_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    status_id BIGINT REFERENCES statuses(id) ON DELETE CASCADE,
    notification_type VARCHAR(50) NOT NULL,
    read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS lists (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    is_private BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS blocks (
    id BIGSERIAL PRIMARY KEY,
    blocker_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    blocked_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(blocker_id, blocked_id)
);

-- Create indexes for other tables
CREATE INDEX IF NOT EXISTS idx_statuses_account_id ON statuses(account_id);
CREATE INDEX IF NOT EXISTS idx_notifications_account_id ON notifications(account_id);
CREATE INDEX IF NOT EXISTS idx_notifications_from_account_id ON notifications(from_account_id);
CREATE INDEX IF NOT EXISTS idx_lists_account_id ON lists(account_id);
CREATE INDEX IF NOT EXISTS idx_blocks_blocker_id ON blocks(blocker_id);
CREATE INDEX IF NOT EXISTS idx_blocks_blocked_id ON blocks(blocked_id);
