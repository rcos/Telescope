CREATE TABLE lost_passwords (
    -- the universally unique identifier for this password recovery.
    recovery_id UUID PRIMARY KEY,
    -- the user id of the user who forgot their password.
    user_id UUID UNIQUE NOT NULL,
    -- when this password recovery expires
    expiration TIMESTAMP WITH TIME ZONE NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id)
);
