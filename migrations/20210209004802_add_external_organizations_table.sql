-- migrate:up

CREATE TABLE external_organizations (
    external_organization_id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    homepage url NOT NULL,
    contact_emails url[] NOT NULL DEFAULT '{}'::url[]
)

-- migrate:down

DROP TABLE external_organizations;