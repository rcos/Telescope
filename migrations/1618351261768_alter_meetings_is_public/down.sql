-- Reverse the effects of up.sql.

-- Recreate the is_public column.
ALTER TABLE meetings ADD COLUMN is_public BOOLEAN NOT NULL DEFAULT TRUE;

-- Set is_public to false on drafts.
UPDATE meetings SET is_public = TRUE WHERE is_draft = FALSE;

-- Remove the is_draft flag.
ALTER TABLE meetings DROP COLUMN is_draft;

-- Re-create the public meetings view.
CREATE VIEW public_meetings AS
SELECT * FROM meetings AS m
WHERE m.is_public = TRUE
ORDER BY m.start_date_time;

COMMENT ON VIEW public_meetings IS 'View for access to public meetings';

-- Re-create original comment on is_public column.
COMMENT ON COLUMN meetings.is_public
IS 'True if it appears on the schedule publicly (can be used for drafts)';
