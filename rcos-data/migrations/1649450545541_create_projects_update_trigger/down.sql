-- Could not auto-generate a down migration.
-- Please write an appropriate down migration for the SQL below:
-- CREATE OR REPLACE FUNCTION update_modified_now() RETURNS trigger AS $update_modified_now$
-- BEGIN
--     IF NEW.update_at IS NULL THEN
--         NEW.update_at := current_timestamp;
--     END IF;
--     return NEW;
-- END;
-- $update_modified_now$ LANGUAGE plpgsql;
--
-- CREATE TRIGGER project_update_modified BEFORE INSERT OR UPDATE on projects
--     FOR EACH ROW EXECUTE FUNCTION update_modified_now();

DROP TRIGGER IF EXISTS project_update_modified on projects;
DROP FUNCTION IF EXISTS update_modified_now();