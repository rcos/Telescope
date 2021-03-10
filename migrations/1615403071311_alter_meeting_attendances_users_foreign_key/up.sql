-- Add user_id column and constraints to meeting attendance.

-- Add column
ALTER TABLE meeting_attendances ADD COLUMN user_id UUID;

-- Set values
UPDATE meeting_attendances
SET user_id = id
FROM users
WHERE meeting_attendances.username = users.username;

-- Add not null constraint
ALTER TABLE meeting_attendances ALTER user_id SET NOT NULL;

-- Add foreign key constraint
ALTER TABLE meeting_attendances ADD FOREIGN KEY (user_id) REFERENCES users(id);

-- Add unique constraint
ALTER TABLE meeting_attendances
ADD CONSTRAINT meeting_attendances_unique_meeting_id_user_id
UNIQUE (meeting_id, user_id);
