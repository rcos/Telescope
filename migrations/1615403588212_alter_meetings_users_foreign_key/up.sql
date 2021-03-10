-- Add host_user_id column and to meetings table

-- Add column
ALTER TABLE meetings ADD COLUMN host_user_id UUID;

-- Set values
UPDATE meetings
SET host_user_id = id
FROM users
WHERE meetings.host_username = users.username;

-- Add foreign key constraint
ALTER TABLE meetings ADD FOREIGN KEY (host_user_id) REFERENCES users(id);
