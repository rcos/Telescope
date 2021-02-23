CREATE TABLE small_group_mentors (
  small_group_id INT NOT NULL REFERENCES small_groups (small_group_id),
  username VARCHAR NOT NULL REFERENCES users (username),

  PRIMARY KEY (small_group_id, username)
);

COMMENT ON TABLE small_group_mentors IS 'Relation between small groups and
users who are the group''s mentors';