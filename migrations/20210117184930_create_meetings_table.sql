-- migrate:up

CREATE TABLE meetings (
  meeting_id SERIAL PRIMARY KEY,
  semester_id VARCHAR NOT NULL REFERENCES semesters (semester_id),
  type meeting_type NOT NULL,
  host_username VARCHAR REFERENCES users (username),
  is_public BOOLEAN DEFAULT true,
  start_date_time TIMESTAMPTZ NOT NULL,
  end_date_time TIMESTAMPTZ NOT NULL,
  title VARCHAR,
  agenda VARCHAR[] DEFAULT '{}',
  presentation_markdown TEXT,
  external_presentation_url url,
  attendance_code VARCHAR,
  recording_url url,
  is_remote BOOLEAN NOT NULL DEFAULT false, -- A default for better days...
  location VARCHAR,
  meeting_url url,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  CHECK ((is_remote AND meeting_url IS NOT NULL)
         OR (NOT is_remote AND location IS NOT NULL)),

  FOREIGN KEY (semester_id, host_username) REFERENCES enrollments (semester_id, username)
);

CREATE INDEX ON meetings (semester_id);
CREATE INDEX ON meetings (type);
CREATE INDEX ON meetings (is_public);
CREATE INDEX ON meetings (start_date_time, end_date_time);

COMMENT ON COLUMN meetings.host_username IS 'Optional host of meeting, e.g.
mentor username for bonus workshop';
COMMENT ON COLUMN meetings.is_public IS 'True if it appears on the schedule
publicly (can be used for drafts)';
COMMENT ON COLUMN meetings.title IS 'Optional meeting title';
COMMENT ON COLUMN meetings.agenda IS 'List of agenda items that will be covered in the meeting';
COMMENT ON COLUMN meetings.presentation_markdown IS 'Markdown for a RevealJS
presentation that is used to auto-generate the presentation';
COMMENT ON COLUMN meetings.external_presentation_url IS 'Link to external
presentation if markdown generated one is not used';
COMMENT ON COLUMN meetings.location IS 'Physical location or URL to join';

-- migrate:down

DROP TABLE meetings;
