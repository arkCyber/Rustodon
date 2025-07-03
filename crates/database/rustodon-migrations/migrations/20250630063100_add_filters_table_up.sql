-- Up migration
CREATE TABLE IF NOT EXISTS filters (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL,
    phrase VARCHAR(255) NOT NULL,
    context VARCHAR(255)[] NOT NULL,
    expires_at TIMESTAMP,
    irreversible BOOLEAN NOT NULL DEFAULT FALSE,
    whole_word BOOLEAN NOT NULL DEFAULT FALSE,
    action VARCHAR(32) NOT NULL DEFAULT 'hide',
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);
