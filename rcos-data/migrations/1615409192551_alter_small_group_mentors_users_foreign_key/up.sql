-- Alter small_group_mentors table to use user ids

-- Add column and foreign key
ALTER TABLE small_group_mentors ADD COLUMN user_id UUID REFERENCES users(id);

-- Set values
UPDATE small_group_mentors
SET user_id = id
FROM users
WHERE small_group_mentors.username = users.username;

-- Add not null constraint
ALTER TABLE small_group_mentors ALTER COLUMN user_id SET NOT NULL;

-- Add unique constraint
ALTER TABLE small_group_mentors
ADD CONSTRAINT unique_small_group_id_user_id
UNIQUE (small_group_id, user_id);
