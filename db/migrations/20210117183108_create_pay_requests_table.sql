-- migrate:up

CREATE TABLE pay_requests (
  semester_id VARCHAR NOT NULL,
  username VARCHAR NOT NULL,
  reason TEXT NOT NULL,
  is_approved BOOLEAN DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (semester_id, username),

  FOREIGN KEY (semester_id, username) REFERENCES enrollments (semester_id, username)
);

COMMENT ON TABLE pay_requests IS 'Users can request to take RCOS for pay INSTEAD
of credit and must be approved.';
COMMENT ON COLUMN pay_requests.reason IS 'The justification for being paid.';

-- migrate:down

DROP TABLE pay_requests;
