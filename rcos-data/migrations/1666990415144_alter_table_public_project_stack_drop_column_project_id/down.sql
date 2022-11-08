comment on column "public"."project_stack"."project_id" is E'Stores the stack associated with a project';
alter table "public"."project_stack" alter column "project_id" drop not null;
alter table "public"."project_stack" add column "project_id" uuid;
