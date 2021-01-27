
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
    -- Below are the actual verifiers
    -- Code (probably auto generated)
    code VARCHAR,
    -- enable attending via link or qr code that goes to link
    enable_link BOOLEAN NOT NULL,
    -- Attendance phrase
    phrase VARCHAR,
    FOREIGN KEY (creator_id) REFERENCES "users" (id)
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

