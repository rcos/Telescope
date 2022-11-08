CREATE TABLE announcements (
  announcement_id SERIAL PRIMARY KEY,
  semester_id varchar NOT NULL REFERENCES semesters(semester_id),
  title varchar NOT NULL,
  body_markdown text NOT NULL,
  close_date_time TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

COMMENT ON TABLE announcements  IS 'Various announcements  made by RCOS';
COMMENT ON COLUMN announcements.title IS 'Short title of announcement';
COMMENT ON COLUMN announcements.body_markdown IS 'Markdown-supported announcement content';
COMMENT ON COLUMN announcements.close_date_time IS 'Date and time the announcement ends';