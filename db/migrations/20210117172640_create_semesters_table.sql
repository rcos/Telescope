-- migrate:up

CREATE TABLE semesters (
  -- Semester ID must be 6 numbers.
  -- This assumes all semesters are after the year 999 and before 10,000
  semester_id VARCHAR(6) PRIMARY KEY CHECK(semester_id ~ '^\d{6}$'),
  title VARCHAR NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL CHECK (end_date > start_date)
);

CREATE INDEX ON semesters (start_date, end_date);

COMMENT ON TABLE semesters IS 'Dates are from official academic calendar:
https://info.rpi.edu/registrar/academic-calendar
A school year has 3 semesters, Spring, Summer, and Fall. Semester IDs are
4-digit starting year + 2-digit start month, e.g. 202009';
COMMENT ON COLUMN semesters.title IS 'Typically season and year, e.g. Fall 2020';
COMMENT ON COLUMN semesters.start_date IS 'Date that classes start';
COMMENT ON COLUMN semesters.end_date IS 'Date that semester ends';

-- migrate:down

DROP TABLE semesters;
