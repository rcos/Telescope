ALTER TABLE projects
    ADD COLUMN is_external BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE projects
    DROP COLUMN external_organization_id;