-- Up migration
CREATE TABLE IF NOT EXISTS filter_keywords (
    id BIGSERIAL PRIMARY KEY,
    filter_id BIGINT NOT NULL,
    keyword VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);
