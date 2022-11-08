-- Add user id column with foreign key constraint to final_grade_appeals table

-- Add user id column and constraint
ALTER TABLE final_grade_appeal ADD COLUMN user_id UUID REFERENCES users(id);

-- Set values
UPDATE final_grade_appeal
SET user_id = id
FROM users
WHERE final_grade_appeal.username = users.username;

-- Add not null constraint
ALTER TABLE final_grade_appeal ALTER COLUMN user_id SET NOT NULL;

-- Add foreign key constraint
ALTER TABLE final_grade_appeal ADD FOREIGN KEY (user_id, semester_id)
    REFERENCES enrollments(user_id, semester_id);
