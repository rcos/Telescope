-- Add constraints to the enrollments table to prevent null values

ALTER TABLE enrollments ALTER COLUMN is_project_lead SET NOT NULL;
ALTER TABLE enrollments ALTER COLUMN is_coordinator SET NOT NULL;
ALTER TABLE enrollments ALTER COLUMN is_for_pay SET NOT NULL;
