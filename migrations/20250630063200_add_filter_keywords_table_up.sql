-- Up migration
CREATE TABLE IF NOT EXISTS filter_keywords (
    id SERIAL PRIMARY KEY,
    filter_id BIGINT NOT NULL,
    keyword VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
