CREATE TYPE syncflow.project_session_status AS ENUM ('Created', 'Started', 'Stopped');

CREATE TABLE syncflow.project_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL,
    comments TEXT,
    empty_timeout INT NOT NULL,
    max_participants INT NOT NULL,
    livekit_room_name VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    status "syncflow"."project_session_status" NOT NULL DEFAULT 'Created',
    project_id UUID NOT NULL REFERENCES syncflow.projects(id) ON DELETE CASCADE
);

-- Ensure the function is created in the syncflow schema
CREATE OR REPLACE FUNCTION syncflow.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create the trigger in the syncflow schema
CREATE TRIGGER update_timestamp
BEFORE UPDATE ON syncflow.project_sessions
FOR EACH ROW
EXECUTE FUNCTION syncflow.update_updated_at_column();
