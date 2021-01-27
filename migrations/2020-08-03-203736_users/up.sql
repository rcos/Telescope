CREATE TABLE "users" (
    -- (universally unique) user id
    id UUID PRIMARY KEY,
    -- name
    name VARCHAR NOT NULL,
    -- profile picture url
    avi_location VARCHAR,
    -- user biography
    bio TEXT NOT NULL DEFAULT '',
    -- github link
    github_link VARCHAR,
    -- chat handle
    chat_handle VARCHAR,
    -- Is this user a sysadmin
    sysadmin BOOLEAN NOT NULL DEFAULT FALSE,
    -- argon2 hashed password of the user.
    -- Theoretically does not need to be 100 but there are not performance
    -- differences.
    hashed_pwd VARCHAR(100) NOT NULL,
    -- when the account was created
    account_created TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Email table.
CREATE TABLE "emails" (
    -- the email itself
    email VARCHAR PRIMARY KEY,
    -- is the email visible to everyone (including onsite admin)
    is_visible BOOLEAN NOT NULL,
    -- user Id associated with email
    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Email confirmations.
CREATE TABLE "confirmations" (
    -- the universally unique invite id
    invite_id UUID UNIQUE NOT NULL,
    -- the email to confirm
    email VARCHAR PRIMARY KEY,
    -- userId (NULL for new account)
    user_id UUID,
    -- when this invite/confirmation expires
    expiration TIMESTAMP WITH TIME ZONE NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Password recovery table
CREATE TABLE "lost_passwords" (
    -- the universally unique identifier for this password recovery.
    recovery_id UUID PRIMARY KEY,
    -- the user id of the user who forgot their password.
    user_id UUID UNIQUE NOT NULL,
    -- when this password recovery expires
    expiration TIMESTAMP WITH TIME ZONE NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);
