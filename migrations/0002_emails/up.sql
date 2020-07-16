-- Your SQL goes here

CREATE TABLE emails (
    -- the email itself
    email VARCHAR(200) PRIMARY KEY,
    -- user Id associated with email
    userId CHAR(36) NOT NULL,
    FOREIGN KEY (userId) REFERENCES users(uuid)
);