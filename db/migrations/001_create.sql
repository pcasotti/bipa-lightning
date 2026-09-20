CREATE TABLE IF NOT EXISTS nodes (
    public_key TEXT PRIMARY KEY,
    alias TEXT NOT NULL,
    capacity BIGINT NOT NULL,
    first_seen TIMESTAMPTZ NOT NULL
);
