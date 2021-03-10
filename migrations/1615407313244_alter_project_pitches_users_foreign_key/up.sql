-- Update project_pitches table to use new user ids

-- Add columns
ALTER TABLE project_pitches ADD COLUMN user_id UUID REFERENCES users(id);
