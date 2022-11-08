CREATE OR REPLACE FUNCTION update_modified_now() RETURNS trigger AS $update_modified_now$
BEGIN
    IF NEW.updated_at = OLD.updated_at THEN
        NEW.updated_at := current_timestamp;
    END IF;
    return NEW;
END;
$update_modified_now$ LANGUAGE plpgsql;

CREATE TRIGGER project_update_modified BEFORE UPDATE on projects
    FOR EACH ROW EXECUTE FUNCTION update_modified_now();
