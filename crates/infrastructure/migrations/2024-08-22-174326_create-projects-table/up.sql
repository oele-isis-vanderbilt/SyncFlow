DROP TYPE IF EXISTS "syncflow"."StorageType" CASCADE;
CREATE TYPE "syncflow"."StorageType" AS ENUM ('S3');

CREATE TABLE syncflow.projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    user_id SERIAL REFERENCES syncflow.users(id) ON DELETE CASCADE,
    name VARCHAR(50) NOT NULL,
    description TEXT,
    
    livekit_server_url TEXT NOT NULL,
    livekit_server_api_key TEXT NOT NULL,
    livekit_server_api_secret TEXT NOT NULL,
    
    storage_type "syncflow"."StorageType" NOT NULL DEFAULT 'S3',
    
    bucket_name VARCHAR(50) NOT NULL,
    endpoint TEXT NOT NULL,
    access_key TEXT NOT NULL,
    secret_key TEXT NOT NULL,
    region VARCHAR(50),

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE (user_id, id)
);
