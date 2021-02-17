CREATE TABLE workshop_proposals (
  workshop_proposal_id SERIAL PRIMARY KEY,
  semester_id VARCHAR NOT NULL REFERENCES semesters (semester_id),
  username VARCHAR NOT NULL REFERENCES users (username),
  topic VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  qualifications VARCHAR NOT NULL,
  first_choice_at TIMESTAMPTZ NOT NULL CHECK (first_choice_at > now()),
  second_choice_at TIMESTAMPTZ NOT NULL CHECK (second_choice_at > now()),
  third_choice_at TIMESTAMPTZ NOT NULL CHECK (third_choice_at > now()),
  reviewer_username VARCHAR REFERENCES users (username),
  reviewer_comments TEXT,
  is_approved BOOLEAN DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  FOREIGN KEY (semester_id, username) REFERENCES enrollments (semester_id, username),
  FOREIGN KEY (semester_id, reviewer_username) REFERENCES enrollments (semester_id, username)
);

CREATE INDEX ON workshop_proposals (semester_id);
CREATE INDEX ON workshop_proposals (username);

COMMENT ON TABLE workshop_proposals IS 'Users (typically mentors) must submit a
proposal to host a workshop and be approved';
COMMENT ON COLUMN workshop_proposals.first_choice_at IS 'First choice for date
and time to host workshop';
COMMENT ON COLUMN workshop_proposals.second_choice_at IS 'Second choice for date
and time to host workshop';
COMMENT ON COLUMN workshop_proposals.third_choice_at IS 'Third choice for date
and time to host workshop';
COMMENT ON COLUMN workshop_proposals.reviewer_username IS 'Username of
coordinator/faculty who reviewed proposal';
COMMENT ON COLUMN workshop_proposals.reviewer_comments IS 'Optional comments left by reviewer';