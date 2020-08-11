CREATE TABLE confirmations (
    -- the universally unique invite id
    invite_id UUID UNIQUE NOT NULL,
    -- the email to confirm
    email VARCHAR(250) PRIMARY KEY,
    -- userId (NULL for new account)
    user_id UUID,
    -- when this invite/confirmation expires
    expiration TIMESTAMP WITH TIME ZONE NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id)
);