CREATE TABLE "public"."project_stack" ("repo" url NOT NULL, "id" serial NOT NULL, PRIMARY KEY ("id") );COMMENT ON TABLE "public"."project_stack" IS E'Stores the stack associated with a project';
