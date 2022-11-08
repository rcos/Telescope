CREATE VIEW public_meetings AS
    SELECT * FROM meetings AS m
    WHERE m.is_public = TRUE
    ORDER BY m.start_date_time;

COMMENT ON VIEW public_meetings IS 'View for access to public meetings';