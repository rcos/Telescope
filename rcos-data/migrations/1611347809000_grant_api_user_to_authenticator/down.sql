-- Revoke permissions on the api user that postgrest uses

REVOKE api_user FROM authenticator;