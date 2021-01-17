-- migrate:up

CREATE VIEW small_group_members AS
    SELECT sg.small_group_id,
        e.*
    FROM enrollments AS e
    JOIN projects AS p ON p.project_id = e.project_id
    JOIN small_group_projects AS sgp ON sgp.project_id = p.project_id
    JOIN small_groups AS sg ON sg.small_group_id = sgp.small_group_id
    WHERE sg.small_group_id = sgid;

COMMENT ON VIEW small_group_members IS 'View for easy access to small group members';

-- migrate:down

DROP VIEW small_group_members;
