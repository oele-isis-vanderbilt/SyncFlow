-- This file should undo anything in `up.sql`
DO $$
    BEGIN
        IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role') THEN
            DROP TYPE Role CASCADE;
        END IF;
        CREATE TYPE Role AS ENUM ('USER', 'ADMIN');
    END
$$;

ALTER TABLE syncflow.users ADD COLUMN role Role NOT NULL DEFAULT 'USER';
