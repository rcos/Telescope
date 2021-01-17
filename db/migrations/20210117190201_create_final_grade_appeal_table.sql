-- migrate:up

CREATE TABLE final_grade_appeal (
  semester_id VARCHAR NOT NULL REFERENCES semesters (semester_id),
  username VARCHAR NOT NULL REFERENCES users (user_id),
  expected_grade VARCHAR NOT NULL, -- TODO is this supposed to be a letter grade?
  reason TEXT NOT NULL,
  is_handled BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (semester_id, username),

  FOREIGN KEY (semester_id, username) REFERENCES enrollments (semester_id, username)
);

COMMENT ON COLUMN final_grade_appeal.expected_grade IS 'Grade the student
expected to receive';
COMMENT ON COLUMN final_grade_appeal.reason IS 'Reason the student believes they
deserve expected_grade';
COMMENT ON COLUMN final_grade_appeal.is_handled IS 'Whether a faculty advisor
has handled this appeal yet.';

-- migrate:down

DROP TABLE final_grade_appeal;
