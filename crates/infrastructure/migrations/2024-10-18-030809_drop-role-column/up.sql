-- Your SQL goes here
ALTER TABLE syncflow.users DROP COLUMN role;

DROP TYPE Role CASCADE;
