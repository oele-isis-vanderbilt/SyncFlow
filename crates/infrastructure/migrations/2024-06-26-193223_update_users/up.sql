-- Your SQL goes here
ALTER TABLE users
ADD COLUMN oauth_provider VARCHAR(255),
ADD COLUMN oauth_provider_user_id VARCHAR(255);

-- Make the password column nullable
ALTER TABLE users
ALTER COLUMN password DROP NOT NULL;

-- Function to enforce constraints
CREATE OR REPLACE FUNCTION enforce_user_constraints()
RETURNS TRIGGER AS $$
BEGIN
    IF (NEW.password IS NOT NULL AND (NEW.oauth_provider IS NOT NULL OR NEW.oauth_provider_user_id IS NOT NULL)) THEN
        RAISE EXCEPTION 'User must have either a password or OAuth details, not both';
    END IF;

    IF (NEW.password IS NULL AND (NEW.oauth_provider IS NULL OR NEW.oauth_provider_user_id IS NULL)) THEN
        RAISE EXCEPTION 'Passwordless OAuth user must have OAuth details';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to call the function before inserting or updating
CREATE TRIGGER user_constraints
BEFORE INSERT OR UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION enforce_user_constraints();
