-- Your SQL goes here
create table syncflow.project_api_keys (
    id SERIAL PRIMARY KEY,
    api_key TEXT NOT NULL UNIQUE,
    api_secret TEXT NOT NULL UNIQUE,
    comments TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    
    user_id SERIAL NOT NULL REFERENCES syncflow.users(id) ON DELETE CASCADE,
    project_id UUID NOT NULL REFERENCES syncflow.projects(id) ON DELETE CASCADE
);