-- Drop user_id and reviewer_id

ALTER TABLE workshop_proposals DROP COLUMN user_id;
ALTER TABLE workshop_proposals DROP COLUMN reviewer_id;
