CREATE TYPE user_role AS ENUM (
  'student',
  'faculty',
  'faculty_advisor',
  'alumn',
  'external',
  'external_mentor'
);

COMMENT ON TYPE user_role IS 'The user''s position within RCOS';

CREATE TYPE user_account AS ENUM (
  'rpi',
  'discord',
  'mattermost',
  'github',
  'gitlab',
  'bitbucket'
);

COMMENT ON TYPE user_account IS 'The website this account is for';

CREATE TYPE meeting_type AS ENUM (
  'large_group',
  'small_group',
  'presentations',
  'bonus_session',
  'grading',
  'mentors',
  'coordinators',
  'other'
);

COMMENT ON TYPE meeting_type IS 'The kind of RCOS meeting this was';

CREATE TYPE chat_association_source AS ENUM (
  'project',
  'small_group'
);

COMMENT ON TYPE chat_association_source IS 'The kind of group this chat is for';

CREATE TYPE chat_association_target AS ENUM (
  'discord_server',
  'discord_text_channel',
  'discord_voice_channel',
  'discord_category',
  'discord_role'
);

COMMENT ON TYPE chat_association_target IS 'The kind of chat that this refers to';

-- https://www.cybertec-postgresql.com/en/postgresql-useful-new-data-types/
CREATE DOMAIN url AS TEXT
       CHECK (VALUE ~ 'https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#()?&//=]*)');

COMMENT ON DOMAIN url IS 'Type that match URLs (http or https)';