CREATE TABLE bonus_attendances (
  bonus_attendance_id SERIAL PRIMARY KEY,
  semester_id VARCHAR NOT NULL REFERENCES semesters (semester_id),
  username VARCHAR NOT NULL REFERENCES users (username),
  reason TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  FOREIGN KEY (semester_id, username) REFERENCES enrollments (semester_id, username)
);

CREATE INDEX ON bonus_attendances (semester_id, username);

COMMENT ON TABLE bonus_attendances IS 'Bonus attendances from different events';