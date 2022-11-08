-- No need for this flag now, just check if external_organization_id is null
ALTER TABLE projects
    DROP COLUMN is_external;

ALTER TABLE projects
    ADD external_organization_id INT REFERENCES external_organizations (external_organization_id);

COMMENT ON COLUMN projects.external_organization_id IS 'Optional external org this project belongs to, e.g. IBM'