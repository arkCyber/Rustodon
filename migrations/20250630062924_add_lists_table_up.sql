-- Up migration
CREATE TABLE IF NOT EXISTS lists (
    id SERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL,
    title VARCHAR(255) NOT NULL,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
