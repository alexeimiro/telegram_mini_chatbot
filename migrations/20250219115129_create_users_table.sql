-- Add migration script here
-- Up migration: Create the users table
CREATE TABLE users (
    user_id BIGINT PRIMARY KEY,          -- Unique Telegram user ID
    username TEXT NOT NULL,              -- Telegram username
    usage_count INTEGER DEFAULT 0,      -- Number of messages sent by the user
    is_subscribed BOOLEAN DEFAULT FALSE -- Subscription status
);

-- Down migration: Drop the users table (for rolling back)
DROP TABLE users;