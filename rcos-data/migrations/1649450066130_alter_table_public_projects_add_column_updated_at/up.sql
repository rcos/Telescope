alter table "public"."projects" add column "updated_at" timestamptz
 default now();
