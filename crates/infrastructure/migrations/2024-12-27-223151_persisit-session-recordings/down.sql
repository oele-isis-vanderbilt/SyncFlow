-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS syncflow.session_egresses;

DROP TYPE IF EXISTS "syncflow"."session_egress_status";

DROP TYPE IF EXISTS "syncflow"."session_egress_type";