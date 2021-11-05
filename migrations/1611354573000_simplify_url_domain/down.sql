-- Remove use of URL type from table's it's in.
ALTER TABLE projects ALTER COLUMN repository_urls TYPE varchar[];

-- Re-define original URL type.
ALTER DOMAIN url DROP CONSTRAINT url_check;
ALTER DOMAIN url ADD CONSTRAINT url_check
    CHECK (VALUE ~ 'https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#()?&//=]*)');

-- Re-define user in projects table
ALTER TABLE projects ALTER COLUMN repository_urls TYPE url[];
