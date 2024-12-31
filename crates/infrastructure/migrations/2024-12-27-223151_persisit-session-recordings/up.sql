-- Your SQL goes here
CREATE TYPE syncflow.session_egress_status AS ENUM (
    'EGRESS_STARTING',
    'EGRESS_ACTIVE', 
    'EGRESS_ENDING', 
    'EGRESS_COMPLETE', 
    'EGRESS_ABORTED', 
    'EGRESS_FAILED', 
    'EGRESS_LIMIT_REACHED'
);

CREATE TYPE syncflow.session_egress_type AS ENUM (
    'ROOM_COMPOSITE',
    'PARTICIPANT',
    'WEB',
    'TRACK_COMPOSITE',
    'TRACK'
);


CREATE TABLE syncflow.session_egresses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    track_id VARCHAR(50) NOT NULL,
    egress_id VARCHAR(50) NOT NULL,
    started_at BIGINT NOT NULL,
    egress_type "syncflow"."session_egress_type" DEFAULT 'TRACK',
    status "syncflow"."session_egress_status" NOT NULL DEFAULT 'EGRESS_STARTING',
    destination TEXT,
    room_name VARCHAR(250) NOT NULL,
    session_id UUID NOT NULL REFERENCES syncflow.project_sessions(id) ON DELETE CASCADE
);