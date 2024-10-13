-- Your SQL goes here
DROP TABLE IF EXISTS syncflow.create_room_actions;

DROP TABLE IF EXISTS syncflow.list_rooms_actions;

DROP TABLE IF EXISTS syncflow.generate_token_actions;

DROP TABLE IF EXISTS syncflow.delete_room_actions;

DROP TYPE IF EXISTS "syncflow"."EgressDestination" CASCADE;
DROP TYPE IF EXISTS "syncflow"."EgressType" CASCADE;

DROP TABLE IF EXISTS syncflow.egress_actions;
