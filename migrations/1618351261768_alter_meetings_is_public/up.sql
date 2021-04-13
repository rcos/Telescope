-- Clarify the draft flag on the meetings table.

ALTER TABLE meetings ADD COLUMN is_draft BOOLEAN NOT NULL DEFAULT FALSE;

-- Indicate that this flag denotes draft status.
COMMENT ON COLUMN meetings.is_draft
IS 'Flag to indicate this meeting is a draft, and the details are not final.';

-- Set the value to true if the old flag was false.
UPDATE meetings SET is_draft = TRUE WHERE is_public = FALSE;

-- Drop the public_meetings view. We could update it, but it's preferable for
-- clients to just use GraphQl filters and ordering in their queries.
DROP VIEW public_meetings;

-- Remove the old flag.
ALTER TABLE meetings DROP COLUMN is_public;
