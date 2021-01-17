-- migrate:up

CREATE TABLE project_presentation_grades (
  semester_id VARCHAR NOT NULL REFERENCES semesters (semester_id),
  project_id INT NOT NULL REFERENCES projects (project_id),
  grader_username VARCHAR NOT NULL REFERENCES users (username),
  grade REAL NOT NULL CHECK (grade >= 0.0),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (semester_id, project_id, grader_username),

  FOREIGN KEY (semester_id, grader_username) REFERENCES enrollments (semester_id, username)
);

COMMENT ON TABLE project_presentation_grades IS 'Grades for end of semester
project presentations. Might need to separate grade Into multiple';

-- migrate:down

DROP TABLE project_presentation_grades;
