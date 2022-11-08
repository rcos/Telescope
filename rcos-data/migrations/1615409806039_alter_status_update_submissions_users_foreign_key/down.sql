-- Drop user_id and grader_id from status_update_submissions

ALTER TABLE status_update_submissions DROP COLUMN user_id;
ALTER TABLE status_update_submissions DROP COLUMN grader_id;
