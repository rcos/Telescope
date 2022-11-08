-- Remove use of url type in projects table.
ALTER TABLE projects ALTER COLUMN repository_urls TYPE varchar[];

-- Simplify definition of url domain.
ALTER DOMAIN url DROP CONSTRAINT url_check;
ALTER DOMAIN url ADD CONSTRAINT url_check
       CHECK (VALUE ~ 'https?:\/\/.+');

-- Re-define column in projects to use url.
ALTER TABLE projects ALTER COLUMN repository_urls TYPE url[];
