-- This file should undo anything in `up.sql`
ALTER TABLE syncflow.users DROP COLUMN first_name,
    DROP COLUMN middle_name,
    DROP COLUMN last_name,
    DROP COLUMN organization,
    DROP COLUMN job_role;