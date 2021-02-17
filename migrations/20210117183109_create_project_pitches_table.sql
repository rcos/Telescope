-- migrate:up

CREATE TABLE project_pitches (
  semester_id VARCHAR NOT NULL REFERENCES semesters (semester_id),
  username VARCHAR NOT NULL REFERENCES users (username),
  existing_project_id INT REFERENCES projects (project_id),
  proposed_title VARCHAR,
  proposed_description TEXT,
  proposed_stack TEXT,
  pitch_slide_url url,
  proposal_url url,
  is_looking_for_members BOOLEAN NOT NULL DEFAULT true,
  is_approved boolean NOT NULL DEFAULT false,
  reviewer_username VARCHAR REFERENCES users (username),
  reviewer_comments TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (semester_id, username),

  FOREIGN KEY (semester_id, username) REFERENCES enrollments (semester_id, username),
  FOREIGN KEY (semester_id, reviewer_username) REFERENCES enrollments (semester_id, username)
);

COMMENT ON TABLE project_pitches IS 'Represents a project pitch by a member at
the start of a semester.  If the pitch is for an existing project, the title,
description, stack can be grabbed. Otherwise, when the proposal is approved
those fields are used to create the actual project';
COMMENT ON COLUMN project_pitches.existing_project_id IS 'Only if pitch for
existing RCOS project';
COMMENT ON COLUMN project_pitches.proposed_title IS 'Null if for existing RCOS project';
COMMENT ON COLUMN project_pitches.pitch_slide_url IS 'Link to 1-slide
presentation for pitch (if they are open)';
COMMENT ON COLUMN project_pitches.proposal_url IS 'Link to semester project proposal';
COMMENT ON COLUMN project_pitches.is_looking_for_members IS 'Open to new members?';
COMMENT ON COLUMN project_pitches.reviewer_comments IS 'Optional notes from graders';

-- migrate:down

DROP TABLE project_pitches;
