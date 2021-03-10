-- Add user_id foreign key to bonus_attendances table

-- Add column to the bonus_attendances table
ALTER TABLE bonus_attendances ADD COLUMN user_id UUID;

-- Set the user id column for all bonus_attendances.
UPDATE bonus_attendances
SET user_id = id
FROM users
WHERE bonus_attendances.username = users.username;

-- User ids should not be null
ALTER TABLE bonus_attendances ALTER user_id SET NOT NULL;

-- Add foreign key constraint.
ALTER TABLE bonus_attendances ADD FOREIGN KEY (user_id) REFERENCES users(id);

-- Create an index similar to the one in the creation migration
CREATE INDEX ON bonus_attendances (user_id, semester_id);