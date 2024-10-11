-- This file should undo anything in `up.sql`

-- Drop the table
DROP TABLE IF EXISTS "users";

-- Drop type Role
DROP TYPE IF EXISTS public."Role" CASCADE;
