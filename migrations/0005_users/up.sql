CREATE TABLE users (
    -- (universally unique) user id
    id UUID PRIMARY KEY,
    -- name
    name VARCHAR(100) NOT NULL,
    -- profile picture url
    avi_location VARCHAR(250),
    -- bcrypt hashed password of the user.
    hashed_pwd CHAR(60) UNIQUE NOT NULL
);