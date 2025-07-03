-- Up migration
CREATE TABLE IF NOT EXISTS statuses (
    id BIGSERIAL PRIMARY KEY,
    uri VARCHAR(255) NOT NULL UNIQUE,
    account_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    visibility VARCHAR(50) NOT NULL DEFAULT 'public',
    sensitive BOOLEAN DEFAULT FALSE,
    spoiler_text VARCHAR(255),
    in_reply_to_id BIGINT,
    in_reply_to_account_id BIGINT,
    reblog_of_id BIGINT,
    application_id BIGINT,
    language VARCHAR(10),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    replies_count BIGINT DEFAULT 0,
    reblogs_count BIGINT DEFAULT 0,
    favourites_count BIGINT DEFAULT 0,
    reblog BOOLEAN DEFAULT FALSE,
    reply BOOLEAN DEFAULT FALSE,
    direct BOOLEAN DEFAULT FALSE
);
