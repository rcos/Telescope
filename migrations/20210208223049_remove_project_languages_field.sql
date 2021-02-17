-- migrate:up
ALTER TABLE projects
    DROP COLUMN languages CASCADE;

-- migrate:down
ALTER TABLE projects
    ADD COLUMN languages VARCHAR[] DEFAULT '{}'::VARCHAR[] NOT NULL;
