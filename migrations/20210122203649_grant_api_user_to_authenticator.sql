-- migrate:up

-- Postgrest can change to api_user role for authenticated requests
GRANT api_user TO authenticator;

-- migrate:down

