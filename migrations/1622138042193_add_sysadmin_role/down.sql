-- Remove user role for sysadmins

-- Recreate the old enum
CREATE TYPE old_user_role AS ENUM ('student', 'faculty', 'faculty_advisor', 'alumn', 'external', 'external_mentor');
COMMENT ON TYPE old_user_role IS 'The user''s position within RCOS';

-- Convert all sysadmin to faculty advisors. They should have similar permissions.
UPDATE users SET role = 'faculty_advisor' WHERE role = 'sysadmin';

-- Drop the faculty_advisors view since it prevents us from changing role type.
DROP VIEW faculty_advisors;

-- Change the user role type
ALTER TABLE users ALTER COLUMN role TYPE old_user_role USING (role::text::old_user_role);

-- Drop the original enum
DROP TYPE user_role;

-- Rename the new type
ALTER TYPE old_user_role RENAME TO user_role;

-- Re-create the faculty advisors view
CREATE VIEW faculty_advisors AS
SELECT u.username, u.preferred_name, u.first_name, u.last_name
FROM users AS u
WHERE u.role = 'faculty_advisor';

COMMENT ON VIEW faculty_advisors IS 'View for access to Faculty Advisors';
