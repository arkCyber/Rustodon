-- Up migration
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    note TEXT,
    avatar_url VARCHAR(2048),
    header_url VARCHAR(2048),
    locked BOOLEAN DEFAULT FALSE,
    bot BOOLEAN DEFAULT FALSE,
    discoverable BOOLEAN DEFAULT TRUE,
    group_account BOOLEAN DEFAULT FALSE,
    last_status_at TIMESTAMP WITH TIME ZONE,
    statuses_count BIGINT DEFAULT 0,
    followers_count BIGINT DEFAULT 0,
    following_count BIGINT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
