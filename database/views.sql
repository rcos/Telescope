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

create or replace
view public_faculty_advisor as
select
    u.username,
    u.preferred_name,
    u.first_name,
    u.last_name
from
    users u
where
    u.role = 'faculty_advisor';