-- Up migration
CREATE TABLE IF NOT EXISTS favourites (
    id SERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL,
    status_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
