-- Undo everything in up.sql.

-- Start by recreating chat association types
CREATE TYPE chat_association_source AS ENUM ('project', 'small_group');
COMMENT ON TYPE chat_association_source IS 'The kind of group this chat is for';

CREATE TYPE chat_association_target AS ENUM (
    'discord_server',
    'discord_text_channel',
    'discord_voice_channel',
    'discord_category',
    'discord_role');
COMMENT ON TYPE chat_association_target IS 'The kind of chat that this refers to';

-- And then recreate the chat associations table.
-- This code is just from the original chat associations migration.
CREATE TABLE chat_associations (
    source_type chat_association_source NOT NULL,
    target_type chat_association_target NOT NULL,
    source_id VARCHAR NOT NULL,
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

-- Merge all data from other tables into chat_associations.

INSERT INTO chat_associations(source_type, target_type, source_id, target_id, created_at)
SELECT 'project', 'discord_text_channel', CAST(project_id AS VARCHAR), channel_id, created_at
FROM project_channels WHERE kind = 'discord_text';

INSERT INTO chat_associations(source_type, target_type, source_id, target_id, created_at)
SELECT 'project', 'discord_voice_channel', CAST(project_id AS VARCHAR), channel_id, created_at
FROM project_channels WHERE kind = 'discord_voice';

INSERT INTO chat_associations(source_type, target_type, source_id, target_id, created_at)
SELECT 'project', 'discord_role', CAST(project_id AS VARCHAR), role_id, created_at
FROM project_roles;

INSERT INTO chat_associations(source_type, target_type, source_id, target_id, created_at)
SELECT 'small_group', 'discord_text_channel', CAST(small_group_id AS VARCHAR), channel_id, created_at
FROM small_group_channels WHERE kind = 'discord_text';

INSERT INTO chat_associations(source_type, target_type, source_id, target_id, created_at)
SELECT 'small_group', 'discord_voice_channel', CAST(small_group_id AS VARCHAR), channel_id, created_at
FROM small_group_channels WHERE kind = 'discord_voice';

INSERT INTO chat_associations(source_type, target_type, source_id, target_id, created_at)
SELECT 'small_group', 'discord_role', CAST(small_group_id AS VARCHAR), role_id, created_at
FROM small_group_roles;

INSERT INTO chat_associations(source_type, target_type, source_id, target_id, created_at)
SELECT 'small_group', 'discord_category', CAST(small_group_id AS VARCHAR), category_id, created_at
FROM small_group_categories;

-- Drop old tables.
DROP TABLE project_channels;
DROP TABLE project_roles;
DROP TABLE small_group_channels;
DROP TABLE small_group_roles;
DROP TABLE small_group_categories;

-- Drop old channel variant type.
DROP TYPE channel_type;
