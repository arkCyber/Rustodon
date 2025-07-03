-- Up migration
CREATE TABLE IF NOT EXISTS list_accounts (
    id SERIAL PRIMARY KEY,
    list_id BIGINT NOT NULL,
    account_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
