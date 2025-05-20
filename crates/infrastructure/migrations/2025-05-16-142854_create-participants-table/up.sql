-- Your SQL goes here
CREATE TABLE syncflow.session_participants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    participant_identity VARCHAR(255) NOT NULL,
    participant_name VARCHAR(255) NOT NULL,
    joined_at BIGINT NOT NULL,
    left_at BIGINT DEFAULT NULL,
    session_id UUID NOT NULL REFERENCES syncflow.project_sessions(id) ON DELETE CASCADE
);

CREATE TYPE syncflow.track_kind as ENUM (
    'AUDIO',
    'VIDEO',
    'UNKNOWN'
);

CREATE TYPE syncflow.track_source as ENUM (
    'CAMERA',
    'SCREEN_SHARE',
    'MICROPHONE',
    'SCREEN_SHARE_AUDIO',
    'UNKNOWN'
);


CREATE TABLE syncflow.participant_tracks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sid VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    kind "syncflow"."track_kind" NOT NULL DEFAULT 'UNKNOWN',
    source "syncflow"."track_source" NOT NULL DEFAULT 'UNKNOWN',
    participant_id UUID NOT NULL REFERENCES syncflow.session_participants(id) ON DELETE CASCADE
);


ALTER TABLE syncflow.session_egresses ADD COLUMN participant_id UUID REFERENCES syncflow.session_participants(id) ON DELETE CASCADE;
ALTER TABLE syncflow.session_egresses ADD COLUMN db_track_id UUID REFERENCES syncflow.participant_tracks(id) ON DELETE CASCADE;
