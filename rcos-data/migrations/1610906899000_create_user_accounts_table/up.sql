CREATE TABLE user_accounts (
  username VARCHAR REFERENCES users (username),
  type user_account,
  account_id VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (username, type)
);

COMMENT ON TABLE user_accounts IS 'User accounts such as Discord, GitHub, GitLab, etc.';
COMMENT ON COLUMN user_accounts.type IS 'Type of external account that is connected';
COMMENT ON COLUMN user_accounts.account_id IS 'Unique ID/username of account';