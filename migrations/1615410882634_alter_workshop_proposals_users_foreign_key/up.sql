-- Use user ids on the workshop proposals table.

-- Add columns and foreign key constraints
ALTER TABLE workshop_proposals ADD COLUMN user_id UUID REFERENCES users(id);
ALTER TABLE workshop_proposals ADD COLUMN reviewer_id UUID REFERENCES users(id);

-- Set values
UPDATE workshop_proposals
SET user_id = id
FROM users
WHERE workshop_proposals.username = users.username;

UPDATE workshop_proposals
SET reviewer_id = id
FROM users
WHERE workshop_proposals.reviewer_username = users.username;

-- Add not null constraint
ALTER TABLE workshop_proposals ALTER COLUMN user_id SET NOT NULL;

-- Add foreign keys referencing enrollments table
ALTER TABLE workshop_proposals
ADD FOREIGN KEY (semester_id, user_id)
REFERENCES enrollments(semester_id, user_id);

ALTER TABLE workshop_proposals
ADD FOREIGN KEY (semester_id, reviewer_id)
REFERENCES enrollments(semester_id, user_id);

-- Create index on user_ids matching the one on usernames
CREATE INDEX ON workshop_proposals(user_id);
