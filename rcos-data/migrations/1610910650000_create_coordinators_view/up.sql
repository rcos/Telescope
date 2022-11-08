CREATE VIEW coordinators AS
    SELECT DISTINCT e.semester_id,
                    u.username,
                    u.preferred_name,
                    u.first_name,
                    u.last_name
    FROM users AS u
    JOIN enrollments AS e ON e.username = u.username
    WHERE e.is_coordinator = TRUE
    ORDER BY e.semester_id, u.username;

COMMENT ON VIEW coordinators IS 'View for access to Coordinators each semester';