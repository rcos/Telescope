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
-- Add comment on host user ID.
COMMENT ON COLUMN meetings.host_user_id IS 'User ID of the optional host of a meeting, for example, the user ID of a mentor hosting a bonus session';

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
-- Add comment on user ID.
COMMENT ON COLUMN mentor_proposals.user_id IS 'User ID of mentor to-be.';

-- Pay requests table.
ALTER TABLE pay_requests DROP CONSTRAINT pay_requests_pkey;
ALTER TABLE pay_requests ADD PRIMARY KEY (semester_id, user_id);
ALTER TABLE pay_requests DROP COLUMN username;

-- Project pitches table.
ALTER TABLE project_pitches DROP CONSTRAINT project_pitches_pkey;
ALTER TABLE project_pitches ADD PRIMARY KEY (semester_id, user_id);
ALTER TABLE project_pitches DROP COLUMN username;
ALTER TABLE project_pitches DROP COLUMN reviewer_username;

-- Project presentation grades table.
ALTER TABLE project_presentation_grades DROP CONSTRAINT project_presentation_grades_pkey;
ALTER TABLE project_presentation_grades ADD PRIMARY KEY (semester_id, project_id, grader_id);
ALTER TABLE project_presentation_grades DROP COLUMN grader_username;
-- Add foreign key to enrollments table.
ALTER TABLE project_presentation_grades ADD FOREIGN KEY (semester_id, grader_id) REFERENCES enrollments(semester_id, user_id);

-- Small group mentors
ALTER TABLE small_group_mentors DROP CONSTRAINT small_group_mentors_pkey;
ALTER TABLE small_group_mentors ADD PRIMARY KEY (small_group_id, user_id);
ALTER TABLE small_group_mentors DROP COLUMN username;

-- Status update submissions
-- Add missing comment.
COMMENT ON COLUMN status_update_submissions.grader_id IS 'The mentor/coordinator/faculty member that graded this status_update';
-- Make primary key changes and drop username column.
ALTER TABLE status_update_submissions DROP CONSTRAINT status_update_submissions_pkey;
ALTER TABLE status_update_submissions ADD PRIMARY KEY (status_update_id, user_id);
ALTER TABLE status_update_submissions DROP COLUMN username;
-- Same with grader.
ALTER TABLE status_update_submissions DROP COLUMN grader_username;

-- Workshop proposal table
-- Add missing comment first.
COMMENT ON COLUMN workshop_proposals.reviewer_id IS 'User ID of coordinator/faculty who reviewed proposal';
-- Drop username and reviewer columns.
ALTER TABLE workshop_proposals DROP COLUMN username;
ALTER TABLE workshop_proposals DROP COLUMN reviewer_username;
-- No need to change pk, it's already just an ID.

-- Enrollments and users table go last because everything else depends on them. 

-- Remove username from enrollments table.
-- First we have to migrate the primary key.
ALTER TABLE enrollments DROP CONSTRAINT enrollments_pkey;
ALTER TABLE enrollments ADD PRIMARY KEY (semester_id, user_id);
-- Then drop the username column.
ALTER TABLE enrollments DROP COLUMN username;

-- Finally the users table.
ALTER TABLE users DROP CONSTRAINT users_pkey;
ALTER TABLE users ADD PRIMARY KEY (id);
ALTER TABLE users DROP COLUMN username;
