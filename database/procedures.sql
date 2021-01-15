create or replace
function get_small_group_members( sgid int ) returns table ( username varchar )
language plpgsql as $$
begin return query
select
	e.username
from
	enrollments e
join projects p on
	p.project_id = e.project_id
join small_group_projects sgp on
	sgp.project_id = p.project_id
join small_groups sg on
	sg.small_group_id = sgp.small_group_id
where
	sg.small_group_id = sgid;
end;$$