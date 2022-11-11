CREATE TABLE users (
  username VARCHAR PRIMARY KEY,
  preferred_name VARCHAR,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  cohort INT,
  role user_role NOT NULL,
  timezone text NOT NULL DEFAULT 'America/New_York',
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

COMMENT ON TABLE users IS 'Users can be students, external mentors, and faculty.
Their user details are not dependent on the semester';
COMMENT ON COLUMN users.preferred_name IS 'Optional preferred first name to use in UIs';
COMMENT ON COLUMN users.first_name IS 'Given name of user';
COMMENT ON COLUMN users.last_name IS 'Family name of user';
COMMENT ON COLUMN users.cohort IS 'Entry year (only set for students)';
COMMENT ON COLUMN users.role IS 'Role of user in RCOS, determines permissions';
COMMENT ON COLUMN users.timezone IS 'Timezone from TZ list';