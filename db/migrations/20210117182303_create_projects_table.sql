-- migrate:up

CREATE TABLE projects (
  project_id SERIAL PRIMARY KEY,
  title VARCHAR UNIQUE NOT NULL,
  description TEXT NOT NULL,
  languages VARCHAR[] NOT NULL DEFAULT '{}',
  stack VARCHAR[] NOT NULL DEFAULT '{}',
  cover_image_url url,
  homepage_url url,
  repository_urls url[] NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TODO indexes for searching

COMMENT ON TABLE projects IS 'Project details are not semester dependent';
COMMENT ON COLUMN projects.languages IS 'List of languages used, all lowercase';
COMMENT ON COLUMN projects.stack IS 'List of technologies used';
COMMENT ON COLUMN projects.cover_image_url IS 'URL to logo image';
COMMENT ON COLUMN projects.homepage_url IS 'Optional link to project homepage';

-- migrate:down

DROP TABLE projects;
