-- Lift those restrictions on the meetings table.

ALTER TABLE meetings ALTER COLUMN is_public DROP NOT NULL;
