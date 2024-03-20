-- Your SQL goes here
CREATE TABLE IF NOT EXISTS key_secret_pairs(
    api_key VARCHAR(255) PRIMARY KEY NOT NULL,
    secret VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    user_id INTEGER NOT NULL,
    comments TEXT DEFAULT '',
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES users(id)
)
