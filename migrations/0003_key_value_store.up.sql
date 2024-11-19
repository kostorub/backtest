-- Add up migration script here
CREATE TABLE IF NOT EXISTS key_value_store (
    key TEXT PRIMARY KEY,            -- Unique key
    value TEXT NOT NULL,             -- Value associated with the key
    expires_at TIMESTAMP NOT NULL             -- Expiration timestamp
);