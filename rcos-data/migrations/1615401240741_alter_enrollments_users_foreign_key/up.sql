-- Add user id column and foreign key constraint to the enrollments table

-- Add column
ALTER TABLE enrollments ADD COLUMN user_id UUID REFERENCES users(id);

-- Set column's values
UPDATE enrollments
SET user_id = id
FROM users
WHERE enrollments.username = users.username;

-- Add not null constraint
ALTER TABLE enrollments ALTER user_id SET NOT NULL;

-- Add unique constraint -- was not sure what to name this honestly, since it
-- would normally be provided by the primary key constraint.
ALTER TABLE enrollments ADD CONSTRAINT enrollments_unique_user_id_semester_id
    UNIQUE (semester_id, user_id);
