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

-- A Group is just a set of users (with a name)
CREATE TABLE "groups" (
    -- The Unique Id of the group
    id UUID PRIMARY KEY,
    -- group name (may be null in case of internal groups)
    name VARCHAR,
    -- does the group have admin privileges (professors and faculty)
    admin BOOLEAN NOT NULL DEFAULT FALSE,
    -- does the group have coordinator privileges
    coordinator BOOLEAN NOT NULL DEFAULT FALSE,
    -- does the group have mentor privileges
    mentor BOOLEAN NOT NULL DEFAULT FALSE
);

-- Members of a group
CREATE TABLE "memberships" (
    -- Group ID
    gid UUID NOT NULL,
    -- User ID
    uid UUID NOT NULL,

    PRIMARY KEY (gid, uid),
    FOREIGN KEY (gid) REFERENCES "groups" (id),
    FOREIGN KEY (uid) REFERENCES "users" (id)
);

-- RCOS Projects
CREATE TABLE "projects" (
    -- Project ID
    id UUID PRIMARY KEY,
    -- PM or Owner
    owner_id UUID NOT NULL,
    -- Membership of the project.
    group_id UUID NOT NULL,
    -- Project Title -- Must be unique to avoid confusion.
    title VARCHAR UNIQUE NOT NULL,
    -- Project Description (CommonMark Markdown)
    description TEXT NOT NULL DEFAULT '',
    -- Is the Project Active
    active BOOLEAN NOT NULL,
    -- Link to Project Repository
    repo_link VARCHAR,
    -- Link to Project Docs
    docs_link VARCHAR,
    -- Link to project blog
    blog_link VARCHAR,

    FOREIGN KEY (owner_id) REFERENCES "users" (id),
    FOREIGN KEY (group_id) REFERENCES "groups" (id)
);

-- Attendance codes
CREATE TABLE "attendance_codes" (
    -- Attendance ID
    id UUID PRIMARY KEY,
    -- Creator user ID
    creator_id UUID NOT NULL,
    -- Created at timestamp (default now)
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    -- Expires at timestamp (default in an hour)
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now() + make_interval(0,0,0,0,1,0,0),
    -- Target group's ID (those who may attend)
    target_id UUID NOT NULL,

    -- Below are the actual verifiers

    -- Code (probably auto generated)
    code VARCHAR,

    -- enable attending via link or qr code that goes to link
    enable_link BOOLEAN NOT NULL,

    -- Attendance phrase
    phrase VARCHAR,

    FOREIGN KEY (creator_id) REFERENCES "users" (id),
    FOREIGN KEY (target_id) REFERENCES "groups" (id)
);

-- Attendance records
CREATE TABLE "attendance_records" (
    -- Student or User ID
    uid UUID NOT NULL,
    -- Attendance ID
    aid UUID NOT NULL,
    -- Value (default FALSE)
    present BOOLEAN NOT NULL DEFAULT FALSE,

    PRIMARY KEY (uid, aid),
    FOREIGN KEY (uid) REFERENCES "users" (id),
    FOREIGN KEY (aid) REFERENCES "attendance_codes" (id)
);

