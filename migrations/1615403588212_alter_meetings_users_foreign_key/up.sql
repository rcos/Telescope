-- Add host_user_id column and to meetings table

-- Add column and constraint
ALTER TABLE meetings ADD COLUMN host_user_id UUID REFERENCES users(id);

-- Set values
UPDATE meetings
SET host_user_id = id
FROM users
WHERE meetings.host_username = users.username;
