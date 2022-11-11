CREATE TABLE chat_associations (
  source_type chat_association_source,
  target_type chat_association_target,
  source_id VARCHAR,
  target_id VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (source_type, target_type, source_id)
);

COMMENT ON TABLE chat_associations IS 'Association of chat platform channel or
other entity with a small group or project';
COMMENT ON COLUMN chat_associations.source_type IS 'What the target is
associated with, e.g. project or small group';
COMMENT ON COLUMN chat_associations.target_type IS 'What the source is
associated with, e.g. Discord TEXT channel';
COMMENT ON COLUMN chat_associations.source_id IS 'ID of source, e.g. project id
or small group id';
COMMENT ON COLUMN chat_associations.target_id IS 'ID of target on platform, e.g.
Discord channel ID';