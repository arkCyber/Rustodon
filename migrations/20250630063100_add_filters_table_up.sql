-- Up migration
CREATE TABLE IF NOT EXISTS filters (
    id SERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL,
    phrase VARCHAR(255) NOT NULL,
    context VARCHAR(255)[] NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE,
    irreversible BOOLEAN NOT NULL DEFAULT FALSE,
    whole_word BOOLEAN NOT NULL DEFAULT FALSE,
    action VARCHAR(32) NOT NULL DEFAULT 'hide',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
