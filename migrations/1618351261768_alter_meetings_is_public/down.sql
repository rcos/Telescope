-- Reverse the effects of up.sql.

-- Recreate the is_public column.
ALTER TABLE meetings ADD COLUMN is_public BOOLEAN NOT NULL DEFAULT TRUE;

-- Set is_public to false on drafts.
UPDATE meetings SET is_public = TRUE WHERE is_draft = FALSE;

-- Remove the is_draft flag.
ALTER TABLE meetings DROP COLUMN is_draft;
