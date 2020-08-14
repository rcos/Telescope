-- Your SQL goes here

CREATE TABLE emails (
    -- the email itself
    email VARCHAR(250) PRIMARY KEY,
    -- user Id associated with email
    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);