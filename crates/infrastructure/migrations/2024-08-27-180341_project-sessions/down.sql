-- Drop the trigger
DROP TRIGGER IF EXISTS update_timestamp ON syncflow.project_sessions;

-- Drop the trigger function
DROP FUNCTION IF EXISTS syncflow.update_updated_at_column();

-- Drop the table
DROP TABLE IF EXISTS syncflow.project_sessions;

-- Drop the type
DROP TYPE IF EXISTS "syncflow"."project_session_status";
