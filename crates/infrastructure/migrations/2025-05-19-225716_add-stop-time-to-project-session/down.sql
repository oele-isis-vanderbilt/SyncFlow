-- This file should undo anything in `up.sql`
ALTER TABLE syncflow.project_sessions DROP COLUMN IF EXISTS stopped_at;
