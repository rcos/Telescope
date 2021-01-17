-- migrate:up

CREATE TABLE status_updates (
  status_update_id SERIAL PRIMARY KEY,
  semester_id VARCHAR NOT NULL REFERENCES semesters (semester_id),
  title VARCHAR,
  open_date_time TIMESTAMPTZ NOT NULL,
  close_date_time TIMESTAMPTZ CHECK ((close_date_time IS NULL) || (close_date_time > open_date_time)),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

COMMENT ON COLUMN status_updates.title IS 'Optional title. If not set, can use open_at date';
COMMENT ON COLUMN status_updates.open_date_time IS 'When submissions start to be accepted';
COMMENT ON COLUMN status_updates.close_date_time IS 'When submissions stop being submittable';

-- migrate:down

DROP TABLE status_updates;
