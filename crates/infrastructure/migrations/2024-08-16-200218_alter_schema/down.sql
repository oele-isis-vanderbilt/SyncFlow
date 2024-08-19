-- Move tables back to the public schema
ALTER TABLE syncflow.users SET SCHEMA public;
ALTER TABLE syncflow.api_keys SET SCHEMA public;
ALTER TABLE syncflow.create_room_actions SET SCHEMA public;
ALTER TABLE syncflow.delete_room_actions SET SCHEMA public;
ALTER TABLE syncflow.egress_actions SET SCHEMA public;
ALTER TABLE syncflow.generate_token_actions SET SCHEMA public;
ALTER TABLE syncflow.list_rooms_actions SET SCHEMA public;
ALTER TABLE syncflow.login_sessions SET SCHEMA public;

-- Recreate the ENUM types in the public schema
CREATE TYPE "KeyType" AS ENUM('Login', 'Api');
CREATE TYPE "EgressDestination" AS ENUM ('S3', 'LocalFile');
CREATE TYPE "EgressType" AS ENUM ('RoomComposite', 'TrackComposite', 'Participant', 'Track', 'Web');

-- Revert changes to the key_type column in the api_keys table
-- Add back the old key_type column with ENUM type in public schema
ALTER TABLE public.api_keys ADD COLUMN key_type_new "KeyType";

-- Migrate data back to the key_type_new column
UPDATE public.api_keys
SET key_type_new = key_type::text::"KeyType";

-- Drop the new key_type column in syncflow schema
ALTER TABLE public.api_keys DROP COLUMN key_type;

-- Rename the key_type_new column back to its original name
ALTER TABLE public.api_keys RENAME COLUMN key_type_new TO key_type;

-- Revert changes to the egress_actions table
-- Add back the old egress_destination and egress_type columns with ENUM types in public schema
ALTER TABLE public.egress_actions ADD COLUMN egress_destination_new "EgressDestination";
ALTER TABLE public.egress_actions ADD COLUMN egress_type_new "EgressType";

-- Migrate data back to the new columns
UPDATE public.egress_actions
SET 
    egress_destination_new = egress_destination::text::"EgressDestination",
    egress_type_new = egress_type::text::"EgressType";

-- Drop the new egress_destination and egress_type columns in syncflow schema
ALTER TABLE public.egress_actions DROP COLUMN egress_destination;
ALTER TABLE public.egress_actions DROP COLUMN egress_type;

-- Rename the new columns back to their original names
ALTER TABLE public.egress_actions RENAME COLUMN egress_destination_new TO egress_destination;
ALTER TABLE public.egress_actions RENAME COLUMN egress_type_new TO egress_type;

-- Drop the ENUM types in the syncflow schema
DROP TYPE IF EXISTS "syncflow"."KeyType" CASCADE;
DROP TYPE IF EXISTS "syncflow"."EgressDestination" CASCADE;
DROP TYPE IF EXISTS "syncflow"."EgressType" CASCADE;


ALTER FUNCTION syncflow.diesel_set_updated_at() SET SCHEMA public;
ALTER FUNCTION syncflow.enforce_user_constraints() SET SCHEMA public;

DROP TRIGGER IF EXISTS user_constraints ON public.users;
DROP TRIGGER IF EXISTS set_updated_at ON public.users;

CREATE TRIGGER user_constraints
BEFORE INSERT OR UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION enforce_user_constraints();

CREATE TRIGGER set_updated_at
BEFORE UPDATE ON users
FOR EACH ROW 
EXECUTE FUNCTION diesel_set_updated_at();


-- Drop the syncflow schema if it's empty
DROP SCHEMA IF EXISTS "syncflow" CASCADE;