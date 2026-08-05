-- Create accounts table
CREATE TABLE IF NOT EXISTS accounts (
    id BLOB PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    access_token TEXT DEFAULT '',
    refresh_token TEXT DEFAULT '',
    expires INTEGER NOT NULL,
    kind TEXT NOT NULL DEFAULT 'offline'
);

-- Create default_user table
CREATE TABLE IF NOT EXISTS default_user (
    user_id BLOB PRIMARY KEY NOT NULL
);

-- Create index on username for faster lookups
CREATE INDEX IF NOT EXISTS idx_accounts_username ON accounts(username);