-- Your SQL goes here
CREATE TABLE syncflow.project_devices(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_name VARCHAR(50) NOT NULL,
    device_group VARCHAR(50) NOT NULL,
    comments TEXT,
    registered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    project_id UUID NOT NULL REFERENCES syncflow.projects(id) ON DELETE CASCADE,
    registered_by INT NOT NULL REFERENCES syncflow.users(id) ON DELETE CASCADE
);