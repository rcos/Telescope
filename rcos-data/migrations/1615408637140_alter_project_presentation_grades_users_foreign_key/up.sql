-- Add user ids to the project presentation grades table.

-- Add column and constraint
ALTER TABLE project_presentation_grades
    ADD COLUMN grader_id UUID REFERENCES users(id);

-- Set values
UPDATE project_presentation_grades
SET grader_id = id
FROM users
WHERE project_presentation_grades.grader_username = users.username;

-- Add not null constraint
ALTER TABLE project_presentation_grades ALTER COLUMN grader_id SET NOT NULL;

-- Add unique constraints to mirror primary key
ALTER TABLE project_presentation_grades
    ADD CONSTRAINT unique_semester_id_project_id_grader_id
        UNIQUE (semester_id, project_id, grader_id);
