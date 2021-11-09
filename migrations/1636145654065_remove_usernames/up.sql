-- Migration to remove usernames from all tables. User-IDs will act as keys from now on.

-- Remove username from bonus attendance table.
ALTER TABLE bonus_attendances DROP COLUMN username;

-- Final grade appeals.
ALTER TABLE final_grade_appeal DROP CONSTRAINT final_grade_appeal_pkey;
ALTER TABLE final_grade_appeal ADD PRIMARY KEY (semester_id, user_id);
ALTER TABLE final_grade_appeal DROP COLUMN username;

-- Meeting attendances
ALTER TABLE meeting_attendances DROP CONSTRAINT meeting_attendances_pkey;
ALTER TABLE meeting_attendances ADD PRIMARY KEY (meeting_id, user_id);
ALTER TABLE meeting_attendances DROP COLUMN username;

-- User accounts table.
ALTER TABLE user_accounts DROP CONSTRAINT user_accounts_pkey;
ALTER TABLE user_accounts ADD PRIMARY KEY (user_id, type);
ALTER TABLE user_accounts DROP COLUMN username;

-- Meetings (hosts).
ALTER TABLE meetings DROP COLUMN host_username;
-- Add foreign key constraint to enrollments table.
ALTER TABLE meetings ADD FOREIGN KEY (host_user_id, semester_id) REFERENCES enrollments(user_id, semester_id);

-- Mentor proposals.
ALTER TABLE mentor_proposals DROP CONSTRAINT mentor_proposals_pkey;
ALTER TABLE mentor_proposals ADD PRIMARY KEY (semester_id, user_id);
ALTER TABLE mentor_proposals DROP COLUMN username;
-- Add missing column and foreign key for reviewer user ID.
ALTER TABLE mentor_proposals ADD COLUMN reviewer_id UUID REFERENCES users(id);
ALTER TABLE mentor_proposals ADD FOREIGN KEY (semester_id, reviewer_id) REFERENCES enrollments(semester_id, user_id);
COMMENT ON COLUMN mentor_proposals.reviewer_id IS 'User ID of coordinator/faculty who reviewed proposal';
-- Set column values.
UPDATE mentor_proposals
SET reviewer_id = id
FROM users
WHERE reviewer_username IS NOT NULL AND reviewer_username = users.username;
-- Drop old reviewer username column.
ALTER TABLE mentor_proposals DROP COLUMN reviewer_username;

-- Enrollments and users table go last because everything else depends on them. 

-- Remove username from enrollments table.
-- First we have to migrate the primary key.
ALTER TABLE enrollments DROP CONSTRAINT enrollments_pkey;
ALTER TABLE enrollments ADD PRIMARY KEY (semester_id, user_id);
-- Then drop the username column.
ALTER TABLE enrollments DROP COLUMN username;
