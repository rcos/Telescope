
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
