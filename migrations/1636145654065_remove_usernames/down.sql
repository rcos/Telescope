-- Replace usernames in the RCOS Database.

-- Make usernames and set them to the string versions of the user IDs.
ALTER TABLE users ADD COLUMN username VARCHAR UNIQUE;
-- Set usernames.
UPDATE users SET username = id::text;
-- Upgrade to pk.
ALTER TABLE users ALTER COLUMN username SET NOT NULL;
ALTER TABLE users DROP CONSTRAINT users_pkey;
ALTER TABLE users ADD PRIMARY KEY (username);

-- Bonus Attendance table.
-- Add back username column.
ALTER TABLE bonus_attendances ADD COLUMN username VARCHAR REFERENCES users(username);
-- Set Username column.
UPDATE bonus_attendances
SET username = users.username
FROM users
WHERE bonus_attendances.user_id = users.id;
-- Ensure not-null.
ALTER TABLE bonus_attendances ALTER COLUMN username SET NOT NULL;

-- Enrollments table.
-- Add username column.
ALTER TABLE enrollments ADD COLUMN username VARCHAR REFERENCES users(username);
-- Set username column.
UPDATE enrollments
SET username = users.username
FROM users
WHERE enrollments.user_id = users.id;
-- Ensure non-null.
ALTER TABLE enrollments ALTER COLUMN username SET NOT NULL;
-- Revert primary key.
ALTER TABLE enrollments DROP CONSTRAINT enrollments_pkey;
ALTER TABLE enrollments ADD PRIMARY KEY (semester_id, username);

-- User accounts table.
-- Add column.
ALTER TABLE user_accounts ADD COLUMN username VARCHAR REFERENCES users(username);
-- Set values.
UPDATE user_accounts
SET username = users.username
FROM users
WHERE users.id = user_accounts.user_id;
-- Constrain non-null.
ALTER TABLE user_accounts ALTER COLUMN username SET NOT NULL;
-- Revert primary key.
ALTER TABLE user_accounts DROP CONSTRAINT user_accounts_pkey;
ALTER TABLE user_accounts ADD PRIMARY KEY (username, type);

-- Final grade appeals
ALTER TABLE final_grade_appeal ADD COLUMN username VARCHAR REFERENCES users(username);
ALTER TABLE final_grade_appeal ADD FOREIGN KEY (username, semester_id) REFERENCES enrollments(username, semester_id);
-- Set usernames.
UPDATE final_grade_appeal
SET username = users.username
FROM users
WHERE users.id = final_grade_appeal.user_id;
-- Set non-null constraint.
ALTER TABLE final_grade_appeal ALTER COLUMN username SET NOT NULL;
-- Revert primary key.
ALTER TABLE final_grade_appeal DROP CONSTRAINT final_grade_appeal_pkey;
ALTER TABLE final_grade_appeal ADD PRIMARY KEY (semester_id, username);

-- Meeting attendances
ALTER TABLE meeting_attendances ADD COLUMN username VARCHAR REFERENCES users(username);
-- Set usernames.
UPDATE meeting_attendances
SET username = users.username
FROM users
WHERE id = meeting_attendances.user_id;
-- Add non-null constraint.
ALTER TABLE meeting_attendances ALTER COLUMN username SET NOT NULL;
-- Primary key.
ALTER TABLE meeting_attendances DROP CONSTRAINT meeting_attendances_pkey;
ALTER TABLE meeting_attendances ADD PRIMARY KEY (meeting_id, username);

-- Meeting (hosts).
ALTER TABLE meetings ADD COLUMN host_username VARCHAR REFERENCES users(username);
COMMENT ON COLUMN meetings.host_username IS 'Optional host of meeting, e.g. mentor username for bonus workshop';
-- Set host usernames.
UPDATE meetings
SET host_username = username
FROM users
WHERE meetings.host_user_id IS NOT NULL AND users.id = meetings.host_user_id;
-- Add foreign key constraint.
ALTER TABLE meetings ADD FOREIGN KEY (host_username, semester_id) REFERENCES enrollments(username, semester_id);

-- Mentor proposals.
ALTER TABLE mentor_proposals ADD COLUMN username VARCHAR REFERENCES users(username);
COMMENT ON COLUMN mentor_proposals.username IS 'Username of mentor to-be.';
ALTER TABLE mentor_proposals ADD COLUMN reviewer_username VARCHAR REFERENCES users(username);
COMMENT ON COLUMN mentor_proposals.reviewer_username IS 'Username of coordinator/faculty who reviewed proposal.';
ALTER TABLE mentor_proposals ADD FOREIGN KEY (username, semester_id) REFERENCES enrollments(username, semester_id);
ALTER TABLE mentor_proposals ADD FOREIGN KEY (reviewer_username, semester_id) REFERENCES enrollments(username, semester_id);
-- Set username.
UPDATE mentor_proposals
SET username = users.username
FROM users
WHERE user_id = users.id;
-- Set reviewer username.
UPDATE mentor_proposals
SET reviewer_username = users.username
FROM users
WHERE mentor_proposals.reviewer_id IS NOT NULL AND mentor_proposals.reviewer_id = users.id;
-- Add not null constraint and then upgrade to primary key.
ALTER TABLE mentor_proposals ALTER COLUMN username SET NOT NULL;
ALTER TABLE mentor_proposals DROP CONSTRAINT mentor_proposals_pkey;
ALTER TABLE mentor_proposals ADD PRIMARY KEY (semester_id, username);
-- Drop reviewer id column.
ALTER TABLE mentor_proposals DROP COLUMN reviewer_id;

-- Pay requests table.
ALTER TABLE pay_requests ADD COLUMN username VARCHAR REFERENCES users(username);
ALTER TABLE pay_requests ADD FOREIGN KEY (username, semester_id) REFERENCES enrollments(username, semester_id);
-- Set usernames.
UPDATE pay_requests
SET username = users.username
FROM users
WHERE user_id = users.id;
-- Set not null, then upgrade to primary
ALTER TABLE pay_requests ALTER COLUMN username SET NOT NULL;
ALTER TABLE pay_requests DROP CONSTRAINT pay_requests_pkey;
ALTER TABLE pay_requests ADD PRIMARY KEY (username, semester_id);

-- Project pitches table.
ALTER TABLE project_pitches ADD COLUMN username VARCHAR REFERENCES users(username);
ALTER TABLE project_pitches ADD COLUMN reviewer_username VARCHAR REFERENCES users(username);
ALTER TABLE project_pitches ADD FOREIGN KEY (username, semester_id) REFERENCES enrollments(username, semester_id);
ALTER TABLE project_pitches ADD FOREIGN KEY (reviewer_username, semester_id) REFERENCES enrollments(username, semester_id);
-- Set usernames.
UPDATE project_pitches
SET username = users.username
FROM users
WHERE user_id = users.id;
-- Set reviewer usernames.
UPDATE project_pitches
SET reviewer_username = users.username
FROM users
WHERE reviewer_id IS NOT NULL AND reviewer_id = users.id;
-- Set not null, upgrade to PK.
ALTER TABLE project_pitches ALTER COLUMN username SET NOT NULL;
ALTER TABLE project_pitches DROP CONSTRAINT project_pitches_pkey;
ALTER TABLE project_pitches ADD PRIMARY KEY (semester_id, username);

-- Project presentation grades table.
ALTER TABLE project_presentation_grades ADD COLUMN grader_username VARCHAR REFERENCES users(username);
ALTER TABLE project_presentation_grades ADD FOREIGN KEY (semester_id, grader_username) REFERENCES enrollments(semester_id, username);
-- Set usernames.
UPDATE project_presentation_grades
SET grader_username = users.username
FROM users
WHERE grader_id = users.id;
-- Upgrade to PK.
ALTER TABLE project_presentation_grades ALTER COLUMN grader_username SET NOT NULL;
ALTER TABLE project_presentation_grades DROP CONSTRAINT project_presentation_grades_pkey;
ALTER TABLE project_presentation_grades ADD PRIMARY KEY (semester_id, project_id, grader_username);

-- Small group mentors.
ALTER TABLE small_group_mentors ADD COLUMN username VARCHAR REFERENCES users(username);
-- Set usernames.
UPDATE small_group_mentors
SET username = users.username
FROM users
WHERE users.id = small_group_mentors.user_id;
-- Upgrade to pk.
ALTER TABLE small_group_mentors ALTER COLUMN username SET NOT NULL;
ALTER TABLE small_group_mentors DROP CONSTRAINT small_group_mentors_pkey;
ALTER TABLE small_group_mentors ADD PRIMARY KEY (small_group_id, username);

-- Status update submissions.
ALTER TABLE status_update_submissions ADD COLUMN username VARCHAR REFERENCES users(username);
ALTER TABLE status_update_submissions ADD COLUMN grader_username VARCHAR REFERENCES users(username);
COMMENT ON COLUMN status_update_submissions.grader_username IS 'The mentor/coordinator/faculty member that graded this status_update';
-- Set username.
UPDATE status_update_submissions
SET username = users.username
FROM users
WHERE user_id = users.id;
-- Set grader username.
UPDATE status_update_submissions
SET grader_username = username
FROM users
WHERE grader_id IS NOT NULL AND grader_id = users.id;
-- Upgrade username to pk.
ALTER TABLE status_update_submissions ALTER COLUMN username SET NOT NULL;
ALTER TABLE status_update_submissions DROP CONSTRAINT status_update_submissions_pkey;
ALTER TABLE status_update_submissions ADD PRIMARY KEY (status_update_id, username);

-- Workshop proposal table
ALTER TABLE workshop_proposals ADD COLUMN username VARCHAR REFERENCES users(username);
ALTER TABLE workshop_proposals ADD COLUMN reviewer_username VARCHAR REFERENCES users(username);
ALTER TABLE workshop_proposals ADD FOREIGN KEY (semester_id, username) REFERENCES enrollments(semester_id, username);
ALTER TABLE workshop_proposals ADD FOREIGN KEY (semester_id, reviewer_username) REFERENCES enrollments(semester_id, username);
COMMENT ON COLUMN workshop_proposals.reviewer_username IS 'Username of coordinator/faculty who reviewed proposal';
-- Set usernames.
UPDATE workshop_proposals
SET username = users.username
FROM users
WHERE user_id = users.id;
-- Set reviewer names.
UPDATE workshop_proposals
SET reviewer_username = users.username
FROM users
WHERE reviewer_id IS NOT NULL AND user_id = users.id;
-- Set username not null.
ALTER TABLE workshop_proposals ALTER COLUMN username SET NOT NULL;
