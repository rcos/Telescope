-- Restrict some columns of the meetings table to be not null.

ALTER TABLE meetings ALTER COLUMN is_public SET NOT NULL;
