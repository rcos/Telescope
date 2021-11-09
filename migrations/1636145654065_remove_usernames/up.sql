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

-- Enrollments and users table go last because everything else depends on them. 

-- Remove username from enrollments table.
-- First we have to migrate the primary key.
ALTER TABLE enrollments DROP CONSTRAINT enrollments_pkey;
ALTER TABLE enrollments ADD PRIMARY KEY (semester_id, user_id);
-- Then drop the username column.
ALTER TABLE enrollments DROP COLUMN username;
