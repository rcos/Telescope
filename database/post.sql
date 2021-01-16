-- Create user that unauthenicated API requests will use. By default can't do anything.
create role web_anon nologin;
grant usage on schema api to web_anon;
grant select on api.public_meetings to web_anon; -- Can read *public* meetings
grant select on api.public_faculty_advisors to web_anon; -- Can read faculty advisors
grant select on api.public_coordinators to web_anon; -- Can read coordinators
grant select on api.projects to web_anon; -- Can read projects
grant select on api.announcements to web_anon; -- Can read announcements

-- Create user that authenticated API requests will use
create role api_user nologin;
grant usage on schema api to api_user;
grant all on api to api_user;

-- Create user that can be logged in for the API
create role authenticator noinherit login password '<password>';
grant web_anon to authenticator;
grant api_user to authenticator;