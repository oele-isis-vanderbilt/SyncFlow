-- This file should undo anything in `up.sql`
CREATE TYPE "syncflow"."EgressDestination" AS ENUM ('S3', 'LocalFile');
CREATE TYPE "syncflow"."EgressType" AS ENUM ('RoomComposite', 'TrackComposite', 'Participant', 'Track', 'Web');


CREATE TABLE IF NOT EXISTS syncflow.create_room_actions (
    id SERIAL PRIMARY KEY,
    room_name TEXT NOT NULL,
    user_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES syncflow.users(id)
);

CREATE TABLE IF NOT EXISTS syncflow.list_rooms_actions (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    listed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES syncflow.users(id)
);

CREATE TABLE IF NOT EXISTS syncflow.generate_token_actions (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    token_identity TEXT NOT NULL,
    token_room TEXT NOT NULL,
    generated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES syncflow.users(id)
);

CREATE TABLE IF NOT EXISTS syncflow.delete_room_actions (
    id SERIAL PRIMARY KEY,
    room_name TEXT NOT NULL,
    user_id INT NOT NULL,
    deleted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES syncflow.users(id)
);


create TABLE IF NOT EXISTS syncflow.egress_actions(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    egress_id TEXT NOT NULL,
    room_name TEXT NOT NULL,
    egress_destination "syncflow"."EgressDestination" NOT NULL,
    egress_type "syncflow"."EgressType" NOT NULL,
    egress_destination_path TEXT NOT NULL,
    egress_destination_root TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES syncflow.users(id)
);
