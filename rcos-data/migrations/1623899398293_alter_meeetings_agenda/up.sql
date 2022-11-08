-- This migration replaces the agenda column of the meetings table
-- with a more general purpose description column.

-- Add description column.
ALTER TABLE meetings ADD COLUMN description TEXT NOT NULL DEFAULT '';
COMMENT ON COLUMN meetings.description IS 'Description of the meeting in CommonMark markdown or plaintext.';

-- Convert all agendas to descriptions. This is the word agenda, followed by
-- a bulleted list of the agenda items on newlines.
UPDATE meetings
SET description = E'Agenda:\n' || array_to_string(agenda, E'\n- ')
WHERE array_length(agenda, 1) > 1;

-- Drop the agenda column.
ALTER TABLE meetings DROP COLUMN agenda;
