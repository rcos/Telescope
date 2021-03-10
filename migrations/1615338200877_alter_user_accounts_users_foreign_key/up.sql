-- This migration updates the foreign key constraint of the user_accounts table
-- to use the id column of the users table.
-- This does not modify any other tables.

-- Add the user_id column
ALTER TABLE user_accounts ADD COLUMN user_id UUID;

-- Set all user ids in the accounts table to the appropriate value.
UPDATE user_accounts
SET user_id = id
FROM users
WHERE user_accounts.username = users.username;

-- Constrain user_account's user_ids to not be null.
ALTER TABLE user_accounts ALTER COLUMN user_id SET NOT NULL;

-- Constrain user_account's user_ids to reference a user's id.
ALTER TABLE user_accounts ADD FOREIGN KEY (user_id) REFERENCES users(id);

-- Add unique constraint for user accounts
ALTER TABLE user_accounts
ADD CONSTRAINT user_accounts_unique_user_id_type
UNIQUE (user_id, type);
