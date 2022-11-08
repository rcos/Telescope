-- Remove NOT NULL constraints on the same columns of the enrollments table

ALTER TABLE enrollments ALTER COLUMN is_project_lead DROP NOT NULL;
ALTER TABLE enrollments ALTER COLUMN is_coordinator DROP NOT NULL;
ALTER TABLE enrollments ALTER COLUMN is_for_pay DROP NOT NULL;
