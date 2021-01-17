-- migrate:up

CREATE TABLE mentor_proposals (
  semester_id VARCHAR,
  username VARCHAR,
  reason TEXT NOT NULL,
  skillset TEXT NOT NULL,
  reviewer_username VARCHAR,
  reviewer_comments TEXT,
  is_approved boolean DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (semester_id, username),

  FOREIGN KEY (semester_id, username) REFERENCES enrollments (semester_id, username),
  FOREIGN KEY (semester_id, reviewer_username) REFERENCES enrollments (semester_id, username)
);

COMMENT ON TABLE mentor_proposals IS 'Users Interested in mentoring each
semester must submit a proposal and be approved.';
COMMENT ON COLUMN mentor_proposals.username IS 'Username of mentor to-be';
COMMENT ON COLUMN mentor_proposals.reason IS 'The reason the user would like to mentor';
COMMENT ON COLUMN mentor_proposals.skillset IS 'Short details of technologies
user can mentor for';
COMMENT ON COLUMN mentor_proposals.reviewer_username IS 'Username of
coordinator/faculty who reviewed proposal';
COMMENT ON COLUMN mentor_proposals.reviewer_comments IS 'Optional comments left by reviewer';
COMMENT ON COLUMN mentor_proposals.is_approved IS 'True if user was approved to
become a mentor for the semester';

-- migrate:down

DROP TABLE mentor_proposals;
