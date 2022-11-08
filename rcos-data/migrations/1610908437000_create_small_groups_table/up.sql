CREATE TABLE small_groups (
  small_group_id SERIAL PRIMARY KEY,
  semester_id VARCHAR NOT NULL REFERENCES semesters (semester_id),
  title VARCHAR NOT NULL,
  location VARCHAR
);

CREATE INDEX ON small_groups (semester_id);
CREATE UNIQUE INDEX ON small_groups (semester_id, title);

COMMENT ON TABLE small_groups IS 'A small group for a specific semester. There
will likely be repeats over semesters only differentiated by semester id.';
COMMENT ON COLUMN small_groups.title IS 'The title of the small group.';
COMMENT ON COLUMN small_groups.location IS 'Possible physical location of small
group, i.e. building and room';