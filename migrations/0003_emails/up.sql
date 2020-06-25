-- Your SQL goes here

CREATE TABLE telescope.telescope.emails (
    -- the email itself
    email VARCHAR(200) PRIMARY KEY,
    -- user Id associated with email
    userId SERIAL NOT NULL,
    FOREIGN KEY (userId) REFERENCES telescope.telescope.users(id)
);