select
	u.first_name || ' ' || u.last_name as name,
	e.username as rcs_id,
	e.credits,
	p.title as project,
	e.is_project_lead as project_lead,
--	false as mentor,
--	count(su) as status_updates,
	coalesce(ta.total_attendances, 0) as total_attendances , -- turns nulls to 0
	0 as presentation_grade
from
	enrollments e
inner join users u on
	u.username = e.username
--left outer join status_updates su on
--	su.username = e.username
--	and su.semester_id = e.semester_id
left outer join total_attendances ta on ta.username = u.username and ta.semester_id = e.semester_id
left outer join projects p on
	p.project_id = e.project_id
where
	e.semester_id = '202009'
