-- This file should undo anything in `up.sql`
-- Drop the trigger
DROP TRIGGER IF EXISTS user_constraints ON users;

-- Drop the function
DROP FUNCTION IF EXISTS enforce_user_constraints;

-- Drop the added columns
ALTER TABLE users
DROP COLUMN IF EXISTS oauth_provider,
DROP COLUMN IF EXISTS oauth_provider_user_id;

-- Revert the password column to NOT NULL
ALTER TABLE users
ALTER COLUMN password SET NOT NULL;