-- Your SQL goes here
DROP TYPE IF EXISTS "EgressDestination" CASCADE;
CREATE TYPE "EgressDestination" AS ENUM ('S3', 'LocalFile');

DROP TYPE IF EXISTS "EgressType" CASCADE;
CREATE TYPE "EgressType" AS ENUM ('RoomComposite', 'TrackComposite', 'Participant', 'Track', 'Web');


create TABLE IF NOT EXISTS egress_actions(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    egress_id TEXT NOT NULL,
    room_name TEXT NOT NULL,
    egress_destination "EgressDestination" NOT NULL,
    egress_type "EgressType" NOT NULL,
    egress_destination_path TEXT NOT NULL,
    egress_destination_root TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id)
);