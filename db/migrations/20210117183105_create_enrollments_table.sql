-- migrate:up

CREATE TABLE enrollments (
  semester_id VARCHAR REFERENCES semesters (semester_id),
  username VARCHAR REFERENCES users (username),
  project_id INT REFERENCES projects (project_id),
  is_project_lead BOOLEAN DEFAULT false,
  is_coordinator BOOLEAN DEFAULT false,
  credits INT NOT NULL DEFAULT 0,
  is_for_pay BOOLEAN DEFAULT false,
  mid_year_grade REAL CHECK (mid_year_grade >= 0.0),
  final_grade REAL CHECK (final_grade >= 0.0),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (semester_id, username)
);

CREATE INDEX ON enrollments (project_id);
CREATE INDEX ON enrollments (credits) WHERE credits > 0;

COMMENT ON TABLE enrollments IS 'An enrollment of a user in RCOS for a specific
semester. They might or might not be on a project and might or might not be
taking RCOS for credit.';
COMMENT ON COLUMN enrollments.is_project_lead IS 'Allows multiple project leads';
COMMENT ON COLUMN enrollments.credits IS '0-4 where 0 means just for experience';
COMMENT ON COLUMN enrollments.is_for_pay IS 'True if taking RCOS for pay';
COMMENT ON COLUMN enrollments.mid_year_grade IS '0.0-100.0';
COMMENT ON COLUMN enrollments.final_grade IS '0.0-100.0';

-- migrate:down

DROP TABLE enrollments;
