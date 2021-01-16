-- View for access to public meetings
create or replace
view public_meetings as
select
	*
from
	meetings m
where
	m.is_public = true
order by
	m.start_date_time;

-- View for access to Faculty Advisors
create or replace
view public_faculty_advisors as
select
    u.username,
    u.preferred_name,
    u.first_name,
    u.last_name
from
    users u
where
    u.role = 'faculty_advisor';

-- View for access to Coordinators each semester
create or replace
view public_coordinators as
select
    distinct
    e.semester_id,
    u.username,
    u.preferred_name,
    u.first_name,
    u.last_name
from
    users u
join enrollments e
    on e.username = u.username
where
    e.is_coordinator = true
order by e.semester_id, u.username;

-- View for access to Mentors for each semester
-- TODO