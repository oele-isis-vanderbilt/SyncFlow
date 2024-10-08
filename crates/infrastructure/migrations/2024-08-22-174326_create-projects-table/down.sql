-- This file should undo anything in `up.sql`
DROP TYPE IF EXISTS "syncflow"."StorageType" CASCADE;
DROP TABLE syncflow.projects;
