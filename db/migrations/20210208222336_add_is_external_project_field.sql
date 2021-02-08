-- migrate:up
ALTER TABLE projects
    ADD COLUMN is_external BOOLEAN NOT NULL DEFAULT FALSE;

-- migrate:down

ALTER TABLE projects
    DROP COLUMN is_external CASCADE;