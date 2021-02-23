CREATE OR REPLACE VIEW duplicate_users
AS
  SELECT DISTINCT
         LEAST(a.username, b.username) AS username_a,
         GREATEST(a.username, b.username)  AS username_b,
         a.first_name,
         a.last_name
  FROM   users AS a
         JOIN users AS b
           ON a.first_name ILIKE b.first_name
              AND a.last_name ILIKE b.last_name
              AND a.username != b.username;
