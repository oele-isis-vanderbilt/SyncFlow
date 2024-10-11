-- Create the syncflow schema if it doesn't exist
CREATE SCHEMA IF NOT EXISTS "syncflow";

-- Move tables from the public schema to the syncflow schema
ALTER TABLE public.users SET SCHEMA "syncflow";
ALTER TABLE public.api_keys SET SCHEMA "syncflow";
ALTER TABLE public.create_room_actions SET SCHEMA "syncflow";
ALTER TABLE public.delete_room_actions SET SCHEMA "syncflow";
ALTER TABLE public.egress_actions SET SCHEMA "syncflow";
ALTER TABLE public.generate_token_actions SET SCHEMA "syncflow";
ALTER TABLE public.list_rooms_actions SET SCHEMA "syncflow";
ALTER TABLE public.login_sessions SET SCHEMA "syncflow";


-- Drop existing types in the syncflow schema if they exist
DROP TYPE IF EXISTS "syncflow"."KeyType" CASCADE;
DROP TYPE IF EXISTS "syncflow"."EgressDestination" CASCADE;
DROP TYPE IF EXISTS "syncflow"."EgressType" CASCADE;

-- Create new ENUM types in the syncflow schema
CREATE TYPE "syncflow"."KeyType" AS ENUM('Login', 'Api');
CREATE TYPE "syncflow"."EgressDestination" AS ENUM ('S3', 'LocalFile');
CREATE TYPE "syncflow"."EgressType" AS ENUM ('RoomComposite', 'TrackComposite', 'Participant', 'Track', 'Web');

-- Alter the key_type column in the api_keys table to use the new ENUM type
ALTER TABLE syncflow.api_keys
    ALTER COLUMN "key_type" DROP DEFAULT;

-- Temporarily rename the key_type column to prevent conflicts during the migration
ALTER TABLE syncflow.api_keys
    RENAME COLUMN "key_type" TO key_type_old;

-- Add a new key_type column with the ENUM type
ALTER TABLE syncflow.api_keys
    ADD COLUMN key_type "syncflow"."KeyType" NOT NULL DEFAULT 'Api';

-- Migrate data to the new key_type column
UPDATE syncflow.api_keys
SET key_type = key_type_old::text::"syncflow"."KeyType";

-- Drop the old key_type column
ALTER TABLE syncflow.api_keys
    DROP COLUMN key_type_old;

-- Set default value for the new key_type column
ALTER TABLE syncflow.api_keys
    ALTER COLUMN key_type SET DEFAULT 'Api';

-- Alter the columns in the egress_actions table to use the new ENUM types
ALTER TABLE syncflow.egress_actions
    ALTER COLUMN egress_destination DROP DEFAULT;
ALTER TABLE syncflow.egress_actions
    ALTER COLUMN egress_type DROP DEFAULT;

-- Temporarily rename the egress_actions columns to prevent conflicts during the migration
ALTER TABLE syncflow.egress_actions
    RENAME COLUMN egress_destination TO egress_destination_old;
ALTER TABLE syncflow.egress_actions
    RENAME COLUMN egress_type TO egress_type_old;

-- Add new columns with the ENUM types
ALTER TABLE syncflow.egress_actions
    ADD COLUMN egress_destination "syncflow"."EgressDestination" NOT NULL DEFAULT 'S3';
ALTER TABLE syncflow.egress_actions
    ADD COLUMN egress_type "syncflow"."EgressType" NOT NULL DEFAULT 'Track';

-- Migrate data to the new columns
UPDATE syncflow.egress_actions
SET
    egress_destination = egress_destination_old::text::"syncflow"."EgressDestination",
    egress_type = egress_type_old::text::"syncflow"."EgressType";

-- Drop the old columns
ALTER TABLE syncflow.egress_actions
    DROP COLUMN egress_destination_old;
ALTER TABLE syncflow.egress_actions
    DROP COLUMN egress_type_old;

-- Set default values for the new columns
ALTER TABLE syncflow.egress_actions
    ALTER COLUMN egress_destination SET DEFAULT 'S3';
ALTER TABLE syncflow.egress_actions
    ALTER COLUMN egress_type SET DEFAULT 'Track';

ALTER FUNCTION public.diesel_set_updated_at() SET SCHEMA syncflow;
ALTER FUNCTION public.enforce_user_constraints() SET SCHEMA syncflow;


DROP TRIGGER IF EXISTS user_constraints ON syncflow.users;
DROP TRIGGER IF EXISTS set_updated_at ON syncflow.users;

CREATE TRIGGER user_constraints
BEFORE INSERT OR UPDATE ON syncflow.users
FOR EACH ROW
EXECUTE FUNCTION syncflow.enforce_user_constraints();

CREATE TRIGGER set_updated_at
BEFORE UPDATE ON syncflow.users
FOR EACH ROW
EXECUTE FUNCTION syncflow.diesel_set_updated_at();

-- Drop existing ENUM types in the public schema if they exist
DROP TYPE IF EXISTS "KeyType" CASCADE;
DROP TYPE IF EXISTS "EgressDestination" CASCADE;
DROP TYPE IF EXISTS "EgressType" CASCADE;
