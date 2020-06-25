-- Your SQL goes here

CREATE TABLE telescope.telescope.users (
    -- user id
    id SERIAL PRIMARY KEY,
    -- name
    name VARCHAR(100) NOT NULL,
    -- profile picture url
    avi_location VARCHAR(250),
    -- SHA-256 hashed password of the user.
    hashed_pwd CHAR(64) UNIQUE NOT NULL
);