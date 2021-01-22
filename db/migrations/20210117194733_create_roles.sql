-- migrate:up

-- Create user that unauthenicated API requests will use. By default can't do anything.
CREATE ROLE web_anon NOLOGIN;
GRANT usage ON SCHEMA public TO web_anon;
GRANT SELECT ON public_meetings TO web_anon; -- Can read *public* meetings
GRANT SELECT ON faculty_advisors TO web_anon; -- Can read faculty advisors
GRANT SELECT ON coordinators TO web_anon; -- Can read coordinators
GRANT SELECT ON projects TO web_anon; -- Can read projects
GRANT SELECT ON announcements TO web_anon; -- Can read announcements

-- Create user that authenticated API requests will use
CREATE ROLE api_user NOLOGIN;
GRANT usage ON SCHEMA public TO api_user;
GRANT ALL ON ALL TABLES IN SCHEMA public TO api_user;
GRANT ALL ON ALL sequences IN SCHEMA public TO api_user;

-- Create user that can be logged in for the API
CREATE ROLE authenticator NOINHERIT LOGIN PASSWORD '<PASSWORD>'; -- MAKE SURE TO CHANGE THIS
GRANT web_anon TO authenticator;

-- migrate:down

-- The order of these must be the reverse of above
DROP OWNED BY authenticator;
DROP OWNED BY api_user;
DROP OWNED BY web_anon;

DROP ROLE authenticator;
DROP ROLE api_user;
DROP ROLE web_anon;
