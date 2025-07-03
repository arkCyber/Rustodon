-- Up migration
CREATE TABLE IF NOT EXISTS reblogs (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL,
    status_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);
