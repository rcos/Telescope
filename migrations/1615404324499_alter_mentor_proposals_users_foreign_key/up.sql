-- Add user_id column and foreign key constraint to mentor_proposals table

-- Add column
ALTER TABLE mentor_proposals ADD COLUMN user_id UUID;

-- Set values
UPDATE mentor_proposals
SET user_id = id
FROM users
WHERE mentor_proposals.username = users.username;

-- Add not null constraint
ALTER TABLE mentor_proposals ALTER user_id SET NOT NULL;

-- Add foreign key to users table
ALTER TABLE mentor_proposals ADD FOREIGN KEY (user_id) REFERENCES users(id);

-- Add unique constraint
ALTER TABLE mentor_proposals
ADD CONSTRAINT mentor_proposals_unique_semsester_id_user_id
UNIQUE (semester_id, user_id);

-- Add foreign key to enrollments table
ALTER TABLE mentor_proposals ADD FOREIGN KEY (semester_id, user_id)
    REFERENCES enrollments(semester_id, user_id);
