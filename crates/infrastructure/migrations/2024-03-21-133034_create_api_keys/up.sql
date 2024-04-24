-- Your SQL goes here
DROP TYPE IF EXISTS "KeyType" CASCADE;
CREATE TYPE "KeyType" AS ENUM ('Login', 'Api');

create TABLE IF NOT EXISTS api_keys (
    id SERIAL PRIMARY KEY,
    key_type "KeyType" NOT NULL,
    key VARCHAR(255) NOT NULL,
    user_id INTEGER NOT NULL,
    secret VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    valid BOOLEAN NOT NULL DEFAULT TRUE,
    comment TEXT DEFAULT NULL,
    CONSTRAINT api_keys_user_id_fkey FOREIGN KEY (user_id) REFERENCES users (id)
)
