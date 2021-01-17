-- migrate:up

CREATE TABLE status_update_submissions (
  status_update_id INT NOT NULL REFERENCES status_updates (status_update_id),
  username VARCHAR NOT NULL REFERENCES users (username),
  this_week TEXT NOT NULL,
  next_week TEXT NOT NULL,
  blockers TEXT NOT NULL,
  grade REAL CHECK (grade >= 0.0),
  grader_username VARCHAR REFERENCES users (username),
  grader_comments TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (status_update_id, username)
);

COMMENT ON TABLE status_update_submissions IS 'A status update submission by a enrolled member';
COMMENT ON COLUMN status_update_submissions.grade IS 'Scale from 0-1: did this
status update meet the requirements';
COMMENT ON COLUMN status_update_submissions.grader_username IS 'The
mentor/coordinator/faculty member that graded this status_update';
COMMENT ON COLUMN status_update_submissions.grader_comments IS 'Given by grader';

-- migrate:down

DROP TABLE status_update_submissions;
