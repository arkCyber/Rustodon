-- Up migration
CREATE TABLE IF NOT EXISTS mutes (
    id SERIAL PRIMARY KEY,
    muter_id BIGINT NOT NULL,
    muted_id BIGINT NOT NULL,
    hide_notifications BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
