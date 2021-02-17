CREATE TABLE small_group_projects (
  small_group_id INT NOT NULL REFERENCES small_groups (small_group_id),
  project_id INT NOT NULL REFERENCES projects (project_id),

  PRIMARY KEY (small_group_id, project_id)
);

COMMENT ON TABLE small_group_projects IS 'Relation between small groups and
projects';