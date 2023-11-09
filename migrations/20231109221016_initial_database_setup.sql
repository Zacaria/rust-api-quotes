-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (
    id UUID PRIMARY KEY,
    book VARCHAR(255) NOT NULL,
    quote TEXT NOT NULL,
    inserted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (book, quote)
)