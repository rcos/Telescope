-- Update project_pitches table to use new user ids

-- Add columns and foreign key constraints
ALTER TABLE project_pitches ADD COLUMN user_id UUID REFERENCES users(id);
ALTER TABLE project_pitches ADD COLUMN reviewer_id UUID REFERENCES users(id);

-- Set values
UPDATE project_pitches
SET user_id = id
FROM users
WHERE project_pitches.username = users.username;

UPDATE project_pitches
SET reviewer_id = id
FROM users
WHERE project_pitches.username = users.username;

-- Add not null constraint to user_id
ALTER TABLE project_pitches ALTER COLUMN user_id SET NOT NULL;

-- Add unique constraint
ALTER TABLE project_pitches
ADD CONSTRAINT project_pitches_unique_semester_id_user_id
UNIQUE (semester_id, user_id);

-- Add foreign key constraints to enrollment table
ALTER TABLE project_pitches
ADD FOREIGN KEY (semester_id, user_id)
REFERENCES enrollments(semester_id, user_id);

ALTER TABLE project_pitches
ADD FOREIGN KEY (semester_id, reviewer_id)
REFERENCES enrollments(semester_id, user_id);
