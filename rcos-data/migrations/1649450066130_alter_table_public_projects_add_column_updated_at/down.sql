-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- alter table "public"."projects" add column "updated_at" timestamptz
--  null;

ALTER table "public"."projects" DROP COLUMN IF EXISTS updated_at;
