-- Add user id column and foreign key constraint to the enrollments table

-- Add column
ALTER TABLE enrollments ADD COLUMN user_id UUID;

-- Set column's values
UPDATE enrollments
SET user_id = id
FROM users
WHERE enrollments.username = users.username;

-- Add not null constraint
ALTER TABLE enrollments ALTER user_id SET NOT NULL;

-- Add foreign key constraint
ALTER TABLE enrollments ADD FOREIGN KEY (user_id) REFERENCES users(id);
