-- Your SQL goes here
ALTER TABLE syncflow.project_sessions ADD COLUMN stopped_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
