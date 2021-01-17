-- migrate:up

CREATE VIEW faculty_advisors AS
    SELECT u.username,
        u.preferred_name,
        u.first_name,
        u.last_name
    FROM users AS u
    WHERE u.role = 'faculty_advisor';

COMMENT ON VIEW faculty_advisors IS 'View for access to Faculty Advisors';

-- migrate:down

DROP VIEW faculty_advisors;
