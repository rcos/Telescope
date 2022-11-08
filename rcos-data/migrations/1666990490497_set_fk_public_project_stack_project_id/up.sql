alter table "public"."project_stack"
  add constraint "project_stack_project_id_fkey"
  foreign key ("project_id")
  references "public"."projects"
  ("project_id") on update restrict on delete cascade;
