-- Your SQL goes here

CREATE TABLE users (
    -- (universally unique) user id
    uuid UUID PRIMARY KEY,
    -- name
    name VARCHAR(100) NOT NULL,
    -- profile picture url
    avi_location VARCHAR(250),
    -- SHA-256 hashed password of the user.
    hashed_pwd CHAR(64) UNIQUE NOT NULL
);