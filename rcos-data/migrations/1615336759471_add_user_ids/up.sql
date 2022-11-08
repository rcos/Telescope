-- Create user ID column and values. Do not alter the database in any other way.
-- Does not remove usernames.

-- Add a column to the user's table as a randomly generated UUID.
ALTER TABLE users ADD COLUMN id UUID UNIQUE NOT NULL DEFAULT gen_random_uuid();
