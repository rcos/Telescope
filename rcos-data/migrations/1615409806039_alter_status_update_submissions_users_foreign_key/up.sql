-- Use user ids in status update submissions

-- Create columns and foreign key constraints
ALTER TABLE status_update_submissions ADD COLUMN user_id UUID REFERENCES users(id);
ALTER TABLE status_update_submissions ADD COLUMN grader_id UUID REFERENCES users(id);

-- Set values
UPDATE status_update_submissions
SET user_id = id
FROM users
WHERE status_update_submissions.username = users.username;

UPDATE status_update_submissions
SET grader_id = id
FROM users
WHERE status_update_submissions.grader_username = users.username;

-- Add not-null constraint
ALTER TABLE status_update_submissions ALTER COLUMN user_id SET NOT NULL;

-- Add unique constraint
ALTER TABLE status_update_submissions
ADD CONSTRAINT unique_status_update_id_user_id
UNIQUE (status_update_id, user_id);
