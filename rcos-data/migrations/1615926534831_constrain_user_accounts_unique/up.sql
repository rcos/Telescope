-- Add unique constraint to user accounts so there cannot be two users linked to
-- the same github/discord/etc.

ALTER TABLE user_accounts
ADD CONSTRAINT unique_type_account_id
UNIQUE (type, account_id);