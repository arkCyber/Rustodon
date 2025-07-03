CREATE TABLE IF NOT EXISTS domain_blocks (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL,
    domain VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(account_id, domain)
);
