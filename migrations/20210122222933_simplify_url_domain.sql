-- migrate:up

-- Remove use of url 
ALTER TABLE projects
    ALTER COLUMN repository_urls
    TYPE varchar[];

ALTER DOMAIN url DROP CONSTRAINT url_check;
ALTER DOMAIN url ADD CONSTRAINT url_check
       CHECK (VALUE ~ 'https?:\/\/.+');

-- Readd use
ALTER TABLE projects
    ALTER COLUMN repository_urls
    TYPE url[];

-- migrate:down

