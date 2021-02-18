CREATE OR REPLACE VIEW duplicate_users
AS
  SELECT a.username AS username_a,
         b.username AS username_b,
         a.first_name,
         a.last_name
  FROM   users AS a
         join users AS b
           ON a.first_name = b.first_name
              AND a.last_name = b.last_name
              AND a.username != b.username;
