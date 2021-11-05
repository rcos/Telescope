-- Replace usernames in the RCOS Database.

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

