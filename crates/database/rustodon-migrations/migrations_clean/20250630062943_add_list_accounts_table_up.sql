-- Up migration
CREATE TABLE IF NOT EXISTS list_accounts (
    id BIGSERIAL PRIMARY KEY,
    list_id BIGINT NOT NULL,
    account_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);
