-- Add user_id foreign key to bonus_attendances table

-- Add user_id column and constraint to the bonus_attendances table
ALTER TABLE bonus_attendances ADD COLUMN user_id UUID REFERENCES users(id);

-- Set the user id column for all bonus_attendances.
UPDATE bonus_attendances
SET user_id = id
FROM users
WHERE bonus_attendances.username = users.username;

-- User ids should not be null
ALTER TABLE bonus_attendances ALTER user_id SET NOT NULL;

-- Create an index similar to the one in the creation migration
CREATE INDEX ON bonus_attendances (user_id, semester_id);
