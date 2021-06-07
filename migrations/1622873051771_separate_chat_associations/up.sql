-- This migration separates the chat associations table out into separate tables
-- based on source type.

-- Create enum to indicate the difference between discord voice and text
-- channels.
CREATE TYPE channel_type AS ENUM ('discord_voice', 'discord_text');
COMMENT ON TYPE channel_type IS 'What kind of Discord channel';

-- We use inner joins through out this file rather than just checking the
-- chat associations source type because the inner join will better guarantee
-- that we don't violate a foreign key constraint.

CREATE TABLE project_channels (
    -- The project that this chat association is for.
    project_id INTEGER NOT NULL REFERENCES projects(project_id),

    -- The ID of this Discord channel.
    channel_id VARCHAR NOT NULL UNIQUE,

    -- The type of channel
    kind channel_type NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),

    PRIMARY KEY (project_id, kind)
);

COMMENT ON TABLE project_channels IS 'The Discord channel IDs associated with projects.';
COMMENT ON COLUMN project_channels.project_id IS 'The RCOS project ID.';
COMMENT ON COLUMN project_channels.channel_id IS 'The Discord channel ID.';

-- Add all the project channels to the new table.
-- First text channels
INSERT INTO project_channels(project_id, channel_id, kind, created_at)
SELECT project_id, target_id, 'discord_text', chat_associations.created_at
FROM projects INNER JOIN chat_associations ON source_id = CAST(project_id AS VARCHAR)
WHERE target_type = 'discord_text_channel' AND source_type = 'project';

-- Then voice channels
INSERT INTO project_channels(project_id, channel_id, kind, created_at)
SELECT project_id, target_id, 'discord_voice', chat_associations.created_at
FROM projects INNER JOIN chat_associations ON source_id = CAST(project_id AS VARCHAR)
WHERE target_type = 'discord_voice_channel' AND source_type = 'project';

-- Next do the same for small group channels
CREATE TABLE small_group_channels (
    small_group_id INTEGER NOT NULL REFERENCES small_groups(small_group_id),
    channel_id VARCHAR UNIQUE NOT NULL,
    kind channel_type NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY (small_group_id, kind)
);

COMMENT ON TABLE small_group_channels IS 'The Discord channel IDs associated with small groups.';
COMMENT ON COLUMN small_group_channels.small_group_id IS 'The ID of the small group.';
COMMENT ON COLUMN small_group_channels.channel_id IS 'The Discord channel ID.';

INSERT INTO small_group_channels(small_group_id, channel_id, kind, created_at)
SELECT small_group_id, target_id, 'discord_voice', chat_associations.created_at
FROM small_groups INNER JOIN chat_associations ON source_id = CAST(small_group_id AS VARCHAR)
WHERE target_type = 'discord_voice_channel' AND source_type = 'small_group';

INSERT INTO small_group_channels(small_group_id, channel_id, kind, created_at)
SELECT small_group_id, target_id, 'discord_text', chat_associations.created_at
FROM small_groups INNER JOIN chat_associations ON source_id = CAST(small_group_id AS VARCHAR)
WHERE target_type = 'discord_text_channel' AND source_type = 'small_group';

-- Do similar for both for roles.
-- Only difference is that there cannot be multiple roles for a project
-- or small group, so no variant flag.

-- Projects first.
CREATE TABLE project_roles (
    project_id INTEGER UNIQUE NOT NULL REFERENCES projects(project_id),
    role_id VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY (project_id, role_id)
);

COMMENT ON TABLE project_roles IS 'Discord roles associated with projects.';
COMMENT ON COLUMN project_roles.project_id IS 'The project ID.';
COMMENT ON COLUMN project_roles.role_id IS 'The Discord role ID.';

INSERT INTO project_roles(project_id, role_id, created_at)
SELECT project_id, target_id, chat_associations.created_at
FROM projects INNER JOIN chat_associations ON source_id = CAST(project_id AS VARCHAR)
WHERE target_type = 'discord_role' AND source_type = 'project';

-- Then small groups
CREATE TABLE small_group_roles (
    small_group_id INTEGER UNIQUE NOT NULL REFERENCES small_groups(small_group_id),
    role_id VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY (small_group_id, role_id)
);

COMMENT ON TABLE small_group_roles IS 'The Discord roles associated with small groups.';
COMMENT ON COLUMN small_group_roles.small_group_id IS 'The associated small group ID.';
COMMENT ON COLUMN small_group_roles.role_id IS 'The Discord role ID.';

INSERT INTO small_group_roles(small_group_id, role_id, created_at)
SELECT small_group_id, target_id, chat_associations.created_at
FROM small_groups INNER JOIN chat_associations ON source_id = CAST(small_group_id AS VARCHAR)
WHERE source_type = 'small_group' AND target_type = 'discord_role';

-- Create table for small group categories. We allow multiple categories per
-- small group for now.
CREATE TABLE small_group_categories (
    small_group_id INTEGER NOT NULL REFERENCES small_groups(small_group_id),
    category_id VARCHAR PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

COMMENT ON TABLE small_group_categories
    IS 'The Discord category IDs associated with small groups.';
COMMENT ON COLUMN small_group_categories.small_group_id IS 'The small group ID.';
COMMENT ON COLUMN small_group_categories.category_id IS 'The Discord category ID.';

INSERT INTO small_group_categories(small_group_id, category_id, created_at)
SELECT small_group_id, target_id, chat_associations.created_at
FROM small_groups INNER JOIN chat_associations ON source_id = CAST(small_group_id AS VARCHAR)
WHERE source_type = 'small_group' AND target_type = 'discord_category';

-- We do not preserve records of Discord servers. There should not be external
-- Discord servers for small groups, and projects using external Discord servers
-- should publicize them in their Discord channels.

-- Drop the old chat associations table and the types associated.
DROP TABLE chat_associations;
DROP TYPE chat_association_source;
DROP TYPE chat_association_target;
