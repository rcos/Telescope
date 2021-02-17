CREATE TABLE meeting_attendances (
  meeting_id INT NOT NULL REFERENCES meetings (meeting_id),
  username VARCHAR NOT NULL REFERENCES users (username),
  is_manually_added BOOLEAN DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (meeting_id, username)
);

COMMENT ON COLUMN meeting_attendances.is_manually_added IS 'True if manually
added by admin and not user';