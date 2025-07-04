-- Up migration
CREATE TABLE IF NOT EXISTS notifications (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL,
    from_account_id BIGINT NOT NULL,
    notification_type VARCHAR(50) NOT NULL,
    status_id BIGINT,
    poll_id BIGINT,
    read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);
