-- This file should undo anything in `up.sql`
ALTER TABLE syncflow.session_egresses DROP COLUMN IF EXISTS db_track_id;
ALTER TABLE syncflow.session_egresses DROP COLUMN participant_id;
DROP TABLE IF EXISTS syncflow.participant_tracks;
DROP TABLE IF EXISTS syncflow.session_participants;
DROP TYPE IF EXISTS "syncflow"."track_kind";
DROP TYPE IF EXISTS "syncflow"."track_source";
