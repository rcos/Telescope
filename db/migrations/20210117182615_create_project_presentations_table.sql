-- migrate:up

CREATE TABLE project_presentations (
  project_id INT NOT NULL REFERENCES projects (project_id),
  semester_id VARCHAR NOT NULL REFERENCES semesters (semester_id),
  presentation_url url NOT NULL,
  is_draft boolean NOT NULL DEFAULT true,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (project_id, semester_id)
);

COMMENT ON TABLE project_presentations IS 'Presentations given by RCOS projects';

-- migrate:down

DROP TABLE project_presentations;
