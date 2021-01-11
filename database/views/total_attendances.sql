-- Final Attendance
-- Takes into account bonus attendances to show a total count for each user across each semester.
-- Takes into account bonus attendances to show a total count for each user across each semester.
-- Might be higher than expected if someone makes all meetings and goes to bonus sessions.
-- When querying this, use least() to cap the attendance count at the max value for the semester.
-- Also, only rows for students who have attendances are shown. When querying you might want to
-- use coalesce(total_attendances.total_attendances, 0) to turn nulls into 0.

create or replace
view total_attendances as
select
	ta.semester_id,
	ta.username,
	count(ta) as total_attendances
from
	(
	select
		m.semester_id,
		ma.username
	from
		meeting_attendance ma
	join meetings m on
		ma.meeting_id = m.meeting_id
union all
	select
		ba.semester_id,
		ba.username
	from
		bonus_attendances ba ) ta
group by
	ta.semester_id,
	ta.username
order by
	semester_id,
	username;