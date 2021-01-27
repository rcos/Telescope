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

    FOREIGN KEY (owner_id) REFERENCES "users" (id),
    FOREIGN KEY (group_id) REFERENCES "groups" (id)
);
