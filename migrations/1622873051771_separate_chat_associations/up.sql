-- This migration separates the chat associations table out into separate tables
-- based on source type.

-- We use inner joins through out this file rather than just checking the
-- chat associations source type because the inner join will guarantee
-- that we don't violate a foreign key constraint.

CREATE TABLE project_channels (
    -- The project that this chat association is for.
    project_id INTEGER NOT NULL REFERENCES projects(project_id),

    -- The ID of this Discord channel.
    channel_id VARCHAR PRIMARY KEY,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

COMMENT ON TABLE project_channels IS 'The Discord channel IDs associated with projects.';
COMMENT ON COLUMN project_channels.project_id IS 'The RCOS project ID.';
COMMENT ON COLUMN project_channels.channel_id IS 'The Discord channel ID.';

-- Add all the project channels to the new table.
INSERT INTO project_channels(project_id, channel_id, created_at)
SELECT projects.project_id, chat_associations.target_id, chat_associations.created_at
FROM projects INNER JOIN chat_associations ON projects.project_id = chat_associations.source_id
WHERE (target_type = 'discord_text_channel' OR target_type = 'discord_voice_channel')
  AND source_type = 'project';

-- Next do the same for small group channels
CREATE TABLE small_group_channels (
    -- Associated small group
    small_group_id INTEGER NOT NULL REFERENCES small_groups(small_group_id),

    -- The ID of the discord channel
    channel_id VARCHAR PRIMARY KEY,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

COMMENT ON TABLE small_group_channels IS 'The Discord channel IDs associated with small groups.';
COMMENT ON COLUMN small_group_channels.small_group_id IS 'The ID of the small group.';
COMMENT ON COLUMN small_group_channels.channel_id IS 'The Discord channel ID.';

INSERT INTO small_group_channels(small_group_id, channel_id, created_at)
SELECT small_group_id, target_id, chat_associations.created_at
FROM small_groups INNER JOIN chat_associations ON small_group_id = source_id
WHERE (target_type = 'discord_voice_channel' OR target_type = 'discord_text_channel')
  AND source_type = 'small_group';

-- Do the same for both for roles.
-- Projects first.
CREATE TABLE project_roles (
    project_id INTEGER NOT NULL REFERENCES projects(project_id),
    role_id VARCHAR PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

COMMENT ON TABLE project_roles IS 'Discord roles associated with projects.';
COMMENT ON COLUMN project_roles.project_id IS 'The project ID.';
COMMENT ON COLUMN project_roles.role_id IS 'The Discord role ID.';

INSERT INTO project_roles(project_id, role_id, created_at)
SELECT project_id, target_id, chat_associations.created_at
FROM projects INNER JOIN chat_associations ON project_id = source_id
WHERE target_type = 'discord_role' AND source_type = 'project';

-- Then small groups
CREATE TABLE small_group_roles (
    small_group_id INTEGER NOT NULL REFERENCES small_groups(small_group_id),
    role_id VARCHAR PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

COMMENT ON TABLE small_group_roles IS 'The Discord roles associated with small groups.';
COMMENT ON COLUMN small_group_roles.small_group_id IS 'The associated small group ID.';
COMMENT ON COLUMN small_group_roles.role_id IS 'The Discord role ID.';

INSERT INTO small_group_roles(small_group_id, role_id, created_at)
SELECT small_group_id, target_id, chat_associations.created_at
FROM small_groups INNER JOIN chat_associations ON small_group_id = source_id
WHERE source_type = 'small_group' AND target_type = 'discord_role';

