-- Up migration
CREATE TABLE IF NOT EXISTS lists (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL,
    title VARCHAR(255) NOT NULL,
    replies_policy VARCHAR(50),
    exclusive BOOLEAN DEFAULT FALSE,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);
