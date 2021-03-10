-- Add user_id column and constraints to pay_requests table

-- Add column
ALTER TABLE pay_requests ADD COLUMN user_id UUID;

-- Set values
UPDATE pay_requests
SET user_id = id
FROM users
WHERE pay_requests.username = users.username;

-- Add not null constraint
ALTER TABLE pay_requests ALTER COLUMN user_id SET NOT NULL;

-- Add users table foreign key constraint
ALTER TABLE pay_requests ADD FOREIGN KEY (user_id) REFERENCES users(id);

-- Add unique constraint
ALTER TABLE pay_requests
ADD CONSTRAINT pay_requests_unique_semester_id_user_id
UNIQUE (semester_id, user_id);

-- Add enrollments foreign key constraint
ALTER TABLE pay_requests
ADD FOREIGN KEY (semester_id, user_id)
REFERENCES enrollments(semester_id, user_id);
