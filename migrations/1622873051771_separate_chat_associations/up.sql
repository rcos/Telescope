-- This migration separates the chat associations table out into separate tables
-- based on source type.

CREATE TABLE project_channels (
    -- The project that this chat association is for.
    project_id INTEGER NOT NULL REFERENCES projects(project_id),

    -- The ID of this Discord channel.
    channel_id VARCHAR NOT NULL UNIQUE,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),

    PRIMARY KEY (project_id, channel_id)
);

COMMENT ON TABLE project_channels IS 'The Discord channel IDs associated with projects.';
COMMENT ON COLUMN project_channels.project_id IS 'The RCOS project ID.';
COMMENT ON COLUMN project_channels.channel_id IS 'The Discord channel ID for this project''s channel. This can be a text channel or voice channel. We do not store which. Clients should query the discord API for this information.';

-- Add all the project channels to the new table.
INSERT INTO project_channels(project_id, channel_id, created_at)
SELECT source_id, target_id, created_at FROM chat_associations
WHERE target_type = 'discord_text_channel' OR target_type = 'discord_voice_channel';
