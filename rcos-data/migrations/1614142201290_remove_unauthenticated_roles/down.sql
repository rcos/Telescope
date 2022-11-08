-- Create user that authenticated API requests will use
CREATE ROLE api_user NOLOGIN;
GRANT usage ON SCHEMA public TO api_user;
GRANT ALL ON ALL TABLES IN SCHEMA public TO api_user;
GRANT ALL ON ALL sequences IN SCHEMA public TO api_user;

-- Create user that can be logged in for the API
CREATE ROLE authenticator NOLOGIN;
GRANT web_anon TO authenticator;
GRANT api_user TO authenticator;

