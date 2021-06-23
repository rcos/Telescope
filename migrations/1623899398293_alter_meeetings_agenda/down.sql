-- Undo up.sql.

-- Add back the agenda column.
ALTER TABLE meetings ADD COLUMN agenda VARCHAR[] DEFAULT '{}'::VARCHAR[];
COMMENT ON COLUMN meetings.agenda IS 'List of agenda items that will be covered in the meeting';

-- Drop description column.
ALTER TABLE meetings DROP COLUMN description;
