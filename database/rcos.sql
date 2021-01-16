-- SQL dump generated using DBML (dbml-lang.org)
-- Database: PostgreSQL
-- Generated at: 2021-01-16T02:28:30.648Z

CREATE TYPE "user_account" AS ENUM (
  'discord',
  'mattermost',
  'github',
  'gitlab',
  'bitbucket'
);

CREATE TYPE "meeting_type" AS ENUM (
  'large_group',
  'small_group',
  'presentations',
  'bonus_session',
  'grading',
  'mentors',
  'coordinators',
  'other'
);

CREATE TYPE "chat_association_source" AS ENUM (
  'project',
  'small_group'
);

CREATE TYPE "chat_association_target" AS ENUM (
  'discord_server',
  'discord_text_channel',
  'discord_voice_channel',
  'discord_category',
  'discord_role'
);

CREATE TABLE "semesters" (
  "semester_id" varchar PRIMARY KEY,
  "title" varchar NOT NULL,
  "start_date" date NOT NULL,
  "end_date" date NOT NULL
);

CREATE TABLE "announcements" (
  "announcement_id" SERIAL PRIMARY KEY,
  "semester_id" varchar NOT NULL,
  "title" varchar NOT NULL,
  "body_markdown" text NOT NULL,
  "close_date_time" timestamp
);

CREATE TABLE "users" (
  "username" varchar PRIMARY KEY,
  "preferred_name" varchar,
  "first_name" varchar NOT NULL,
  "last_name" varchar NOT NULL,
  "graduation_year" int,
  "is_rpi" boolean DEFAULT true,
  "is_faculty" boolean DEFAULT false,
  "timezone" text NOT NULL DEFAULT 'America/New_York'
);

CREATE TABLE "user_accounts" (
  "username" varchar,
  "type" user_account,
  "account_id" varchar NOT NULL,
  PRIMARY KEY ("username", "type")
);

CREATE TABLE "mentor_proposals" (
  "semester_id" varchar,
  "username" varchar,
  "reason" text NOT NULL,
  "skillset" text NOT NULL,
  "reviewer_username" varchar,
  "reviewer_comments" text,
  "is_approved" boolean DEFAULT false,
  "submitted_at" timestamp NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  PRIMARY KEY ("semester_id", "username")
);

CREATE TABLE "workshop_proposals" (
  "workshop_proposal_id" SERIAL PRIMARY KEY,
  "semester_id" varchar NOT NULL,
  "username" varchar NOT NULL,
  "topic" varchar NOT NULL,
  "title" varchar NOT NULL,
  "qualifications" varchar NOT NULL,
  "first_choice_at" timestamp NOT NULL,
  "second_choice_at" timestamp NOT NULL,
  "third_datetime_at" timestamp NOT NULL,
  "reviewer_username" varchar,
  "reviewer_comments" text,
  "is_approved" boolean DEFAULT false,
  "submitted_at" timestamp NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);

CREATE TABLE "pay_requests" (
  "semester_id" varchar,
  "username" varchar,
  "reason" varchar NOT NULL,
  "is_approved" boolean DEFAULT false,
  "submitted_at" timestamp NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  PRIMARY KEY ("semester_id", "username")
);

CREATE TABLE "projects" (
  "project_id" SERIAL PRIMARY KEY,
  "title" varchar UNIQUE NOT NULL,
  "description" text NOT NULL,
  "languages" varchar[] NOT NULL DEFAULT '{}',
  "stack" varchar[] NOT NULL DEFAULT '{}',
  "cover_image_url" varchar,
  "homepage_url" varchar,
  "repository_url" varchar NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);

CREATE TABLE "project_presentations" (
  "project_id" int,
  "semester_id" varchar,
  "presentation_url" varchar NOT NULL,
  "is_draft" boolean NOT NULL DEFAULT true,
  PRIMARY KEY ("project_id", "semester_id")
);

CREATE TABLE "project_pitches" (
  "semester_id" varchar,
  "username" varchar,
  "existing_project_id" int,
  "proposed_title" varchar,
  "proposed_description" text,
  "proposed_stack" varchar,
  "pitch_slide_url" varchar,
  "proposal_url" varchar,
  "is_looking_for_members" boolean,
  "is_approved" boolean NOT NULL DEFAULT false,
  "reviewer_username" varchar,
  "reviewer_comments" text,
  "submitted_at" timestamp NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  PRIMARY KEY ("semester_id", "username")
);

CREATE TABLE "enrollments" (
  "semester_id" varchar,
  "username" varchar,
  "project_id" int,
  "is_project_lead" boolean DEFAULT false,
  "is_coordinator" boolean DEFAULT false,
  "credits" int NOT NULL,
  "is_for_pay" boolean DEFAULT false,
  "mid_year_grade" real,
  "final_grade" real,
  "enrolled_at" timestamp NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  PRIMARY KEY ("semester_id", "username")
);

CREATE TABLE "small_groups" (
  "small_group_id" SERIAL PRIMARY KEY,
  "semester_id" varchar NOT NULL,
  "title" varchar NOT NULL,
  "location" varchar
);

CREATE TABLE "small_group_projects" (
  "small_group_id" int,
  "project_id" int,
  PRIMARY KEY ("small_group_id", "project_id")
);

CREATE TABLE "small_group_mentors" (
  "small_group_id" int,
  "username" varchar,
  PRIMARY KEY ("small_group_id", "username")
);

CREATE TABLE "status_update_submissions" (
  "status_update_id" int,
  "username" varchar,
  "this_week" text NOT NULL,
  "next_week" text NOT NULL,
  "blockers" text NOT NULL,
  "submitted_at" timestamp NOT NULL,
  "grade" real,
  "grader_username" varchar,
  "grader_comments" text,
  PRIMARY KEY ("status_update_id", "username")
);

CREATE TABLE "status_updates" (
  "status_update_id" SERIAL PRIMARY KEY,
  "semester_id" varchar NOT NULL,
  "title" varchar,
  "open_date_time" timestamp NOT NULL,
  "close_date_time" timestamp
);

CREATE TABLE "meetings" (
  "meeting_id" SERIAL PRIMARY KEY,
  "semester_id" varchar NOT NULL,
  "meeting_type" meeting_type NOT NULL,
  "host_username" varchar,
  "is_public" boolean DEFAULT true,
  "start_date_time" timestamp NOT NULL,
  "end_date_time" timestamp NOT NULL,
  "title" varchar,
  "agenda" varchar[] DEFAULT '{}',
  "presentation_markdown" text,
  "presentation_url" varchar,
  "attendance_code" varchar,
  "recording_url" varchar,
  "location" varchar
);

CREATE TABLE "meeting_attendances" (
  "meeting_id" int,
  "username" varchar,
  "is_manually_added" boolean DEFAULT false,
  "submitted_at" timestamp NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  PRIMARY KEY ("meeting_id", "username")
);

CREATE TABLE "bonus_attendances" (
  "bonus_attendance_id" SERIAL PRIMARY KEY,
  "semester_id" varchar,
  "username" varchar,
  "reason" varchar NOT NULL,
  "submitted_at" timestamp DEFAULT (CURRENT_TIMESTAMP)
);

CREATE TABLE "project_presentation_grades" (
  "semester_id" varchar,
  "project_id" int,
  "grader_username" varchar,
  "grade" real NOT NULL,
  "submitted_at" timestamp NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  PRIMARY KEY ("semester_id", "project_id", "grader_username")
);

CREATE TABLE "chat_associations" (
  "source_type" chat_association_source,
  "target_type" chat_association_target,
  "source_id" varchar,
  "target_id" varchar NOT NULL,
  PRIMARY KEY ("source_type", "target_type", "source_id")
);

CREATE TABLE "final_grade_appeal" (
  "semester_id" varchar,
  "username" varchar,
  "expected_grade" varchar NOT NULL,
  "reason" text NOT NULL,
  "is_handled" boolean NOT NULL DEFAULT false,
  "submitted_at" timestamp NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  PRIMARY KEY ("semester_id", "username")
);

ALTER TABLE "project_pitches" ADD FOREIGN KEY ("existing_project_id") REFERENCES "projects" ("project_id");

ALTER TABLE "enrollments" ADD FOREIGN KEY ("semester_id") REFERENCES "semesters" ("semester_id");

ALTER TABLE "enrollments" ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE "enrollments" ADD FOREIGN KEY ("project_id") REFERENCES "projects" ("project_id");

ALTER TABLE "small_groups" ADD FOREIGN KEY ("semester_id") REFERENCES "semesters" ("semester_id");

ALTER TABLE "small_group_projects" ADD FOREIGN KEY ("small_group_id") REFERENCES "small_groups" ("small_group_id");

ALTER TABLE "small_group_projects" ADD FOREIGN KEY ("project_id") REFERENCES "projects" ("project_id");

ALTER TABLE "small_group_mentors" ADD FOREIGN KEY ("small_group_id") REFERENCES "small_groups" ("small_group_id");

ALTER TABLE "small_group_mentors" ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE "status_updates" ADD FOREIGN KEY ("semester_id") REFERENCES "semesters" ("semester_id");

ALTER TABLE "meetings" ADD FOREIGN KEY ("semester_id") REFERENCES "semesters" ("semester_id");

ALTER TABLE "project_pitches" ADD FOREIGN KEY ("semester_id", "username") REFERENCES "enrollments" ("semester_id", "username");

ALTER TABLE "project_pitches" ADD FOREIGN KEY ("semester_id", "reviewer_username") REFERENCES "enrollments" ("semester_id", "username");

ALTER TABLE "bonus_attendances" ADD FOREIGN KEY ("semester_id", "username") REFERENCES "enrollments" ("semester_id", "username");

ALTER TABLE "meetings" ADD FOREIGN KEY ("semester_id", "host_username") REFERENCES "enrollments" ("semester_id", "username");

ALTER TABLE "final_grade_appeal" ADD FOREIGN KEY ("semester_id", "username") REFERENCES "enrollments" ("semester_id", "username");

ALTER TABLE "meeting_attendances" ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE "meeting_attendances" ADD FOREIGN KEY ("meeting_id") REFERENCES "meetings" ("meeting_id");

ALTER TABLE "meetings" ADD FOREIGN KEY ("host_username") REFERENCES "users" ("username");

ALTER TABLE "mentor_proposals" ADD FOREIGN KEY ("semester_id", "username") REFERENCES "enrollments" ("semester_id", "username");

ALTER TABLE "workshop_proposals" ADD FOREIGN KEY ("semester_id", "username") REFERENCES "enrollments" ("semester_id", "username");

ALTER TABLE "pay_requests" ADD FOREIGN KEY ("semester_id", "username") REFERENCES "enrollments" ("semester_id", "username");

ALTER TABLE "mentor_proposals" ADD FOREIGN KEY ("semester_id", "reviewer_username") REFERENCES "enrollments" ("semester_id", "username");

ALTER TABLE "workshop_proposals" ADD FOREIGN KEY ("semester_id", "reviewer_username") REFERENCES "enrollments" ("semester_id", "username");

ALTER TABLE "status_update_submissions" ADD FOREIGN KEY ("status_update_id") REFERENCES "status_updates" ("status_update_id");

ALTER TABLE "status_update_submissions" ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE "status_update_submissions" ADD FOREIGN KEY ("grader_username") REFERENCES "users" ("username");

ALTER TABLE "project_presentation_grades" ADD FOREIGN KEY ("project_id") REFERENCES "projects" ("project_id");

ALTER TABLE "project_presentation_grades" ADD FOREIGN KEY ("semester_id") REFERENCES "semesters" ("semester_id");

ALTER TABLE "project_presentation_grades" ADD FOREIGN KEY ("semester_id", "grader_username") REFERENCES "enrollments" ("semester_id", "username");

ALTER TABLE "user_accounts" ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE "project_presentations" ADD FOREIGN KEY ("project_id") REFERENCES "projects" ("project_id");

ALTER TABLE "project_presentations" ADD FOREIGN KEY ("semester_id") REFERENCES "semesters" ("semester_id");

CREATE INDEX ON "semesters" ("start_date", "end_date");

CREATE INDEX ON "workshop_proposals" ("semester_id");

CREATE INDEX ON "workshop_proposals" ("username");

CREATE INDEX ON "enrollments" ("project_id");

CREATE INDEX ON "enrollments" ("credits");

CREATE INDEX ON "small_groups" ("semester_id");

CREATE UNIQUE INDEX ON "small_groups" ("semester_id", "title");

CREATE INDEX ON "meetings" ("semester_id");

CREATE INDEX ON "meetings" ("start_date_time", "end_date_time");

CREATE INDEX ON "bonus_attendances" ("semester_id", "username");

COMMENT ON TABLE "semesters" IS 'Dates are from official academic calendar: https://info.rpi.edu/registrar/academic-calendar
A school year has 3 semesters, Spring, Summer, and Fall. Semester IDs are 4-digit starting year + 2-digit start month, e.g. "202009"';

COMMENT ON COLUMN "semesters"."title" IS 'Typically season and year, e.g. "Fall 2020"';

COMMENT ON COLUMN "announcements"."close_date_time" IS 'Date and time the announcement ends.';

COMMENT ON TABLE "users" IS 'Users can be students, external mentors, and faculty. Their user details are not dependent on the semester.';

COMMENT ON COLUMN "users"."username" IS 'Will be RCS ID unless outside of RPI';

COMMENT ON COLUMN "users"."preferred_name" IS 'Optional preferred first name to use in UIs';

COMMENT ON COLUMN "users"."graduation_year" IS 'Only set for RPI students';

COMMENT ON COLUMN "users"."is_rpi" IS 'True if current student or faculty at RPI';

COMMENT ON COLUMN "users"."is_faculty" IS 'True if faculty at RPI';

COMMENT ON COLUMN "users"."timezone" IS 'Timezone from TZ list';

COMMENT ON TABLE "user_accounts" IS 'User accounts such as Discord, GitHub, GitLab, etc.';

COMMENT ON COLUMN "user_accounts"."account_id" IS 'ID/username of account';

COMMENT ON TABLE "mentor_proposals" IS 'Users interested in mentoring each semester must submit a proposal and be approved.';

COMMENT ON COLUMN "mentor_proposals"."username" IS 'Username of mentor to-be';

COMMENT ON COLUMN "mentor_proposals"."reason" IS 'The reason the user would like to mentor';

COMMENT ON COLUMN "mentor_proposals"."skillset" IS 'Short details of technologies user can mentor for';

COMMENT ON COLUMN "mentor_proposals"."reviewer_username" IS 'Username of coordinator/faculty who reviewed proposal';

COMMENT ON COLUMN "mentor_proposals"."reviewer_comments" IS 'Optional comments left by reviewer';

COMMENT ON COLUMN "mentor_proposals"."is_approved" IS 'True if user was approved to become a mentor for the semester';

COMMENT ON TABLE "workshop_proposals" IS 'Users (typically mentors) must submit a proposal to host a workshop and be approved.';

COMMENT ON COLUMN "workshop_proposals"."first_choice_at" IS 'First choice for date and time to host workshop';

COMMENT ON COLUMN "workshop_proposals"."second_choice_at" IS 'Second choice for date and time to host workshop';

COMMENT ON COLUMN "workshop_proposals"."third_datetime_at" IS 'Third choice for date and time to host workshop';

COMMENT ON COLUMN "workshop_proposals"."reviewer_username" IS 'Username of coordinator/faculty who reviewed proposal';

COMMENT ON COLUMN "workshop_proposals"."reviewer_comments" IS 'Optional comments left by reviewer';

COMMENT ON TABLE "pay_requests" IS 'Users can request to take RCOS for pay INSTEAD of credit and must be approved.';

COMMENT ON COLUMN "pay_requests"."reason" IS 'The justification for being paid.';

COMMENT ON TABLE "projects" IS 'Project details are not semester dependent.';

COMMENT ON COLUMN "projects"."languages" IS 'List of languages used, all lowercase';

COMMENT ON COLUMN "projects"."stack" IS 'List of technologies used';

COMMENT ON COLUMN "projects"."cover_image_url" IS 'URL to logo image';

COMMENT ON COLUMN "projects"."homepage_url" IS 'Optional link to project homepage';

COMMENT ON TABLE "project_pitches" IS 'Represents a project pitch by a member at the start of a semester.
If the pitch is for an existing project, the title, description, stack
can be grabbed. Otherwise, when the proposal is approved those fields
are used to create the actual project.';

COMMENT ON COLUMN "project_pitches"."existing_project_id" IS 'Only if pitch for existing RCOS project';

COMMENT ON COLUMN "project_pitches"."proposed_title" IS 'Null if for existing RCOS project';

COMMENT ON COLUMN "project_pitches"."pitch_slide_url" IS 'Link to 1-slide presentation for pitch (if they are open)';

COMMENT ON COLUMN "project_pitches"."proposal_url" IS 'Link to semester project proposal';

COMMENT ON COLUMN "project_pitches"."is_looking_for_members" IS 'Open to new members?';

COMMENT ON COLUMN "project_pitches"."reviewer_comments" IS 'Optional notes from graders';

COMMENT ON TABLE "enrollments" IS 'An enrollment of a user in RCOS for a specific semester. They might or might not be on a project and might or might not be taking RCOS for credit.';

COMMENT ON COLUMN "enrollments"."is_project_lead" IS 'Allows multiple project leads';

COMMENT ON COLUMN "enrollments"."credits" IS '0-4 where 0 means just for experience';

COMMENT ON COLUMN "enrollments"."is_for_pay" IS 'True if taking RCOS for pay';

COMMENT ON COLUMN "enrollments"."mid_year_grade" IS '0.0-100.0';

COMMENT ON COLUMN "enrollments"."final_grade" IS '0.0-100.0';

COMMENT ON TABLE "small_groups" IS 'A small group for a specific semester. There will likely be repeats over semesters only differentiated by semester id.';

COMMENT ON COLUMN "small_groups"."title" IS 'The title of the small group.';

COMMENT ON COLUMN "small_groups"."location" IS 'Possible physical location of small group, i.e. building and room';

COMMENT ON TABLE "status_update_submissions" IS 'A status update submission by a enrolled member.';

COMMENT ON COLUMN "status_update_submissions"."grade" IS 'Scale from 0-1: did this status update meet the requirements.';

COMMENT ON COLUMN "status_update_submissions"."grader_username" IS 'The mentor/coordinator/faculty member that graded this status_update.';

COMMENT ON COLUMN "status_update_submissions"."grader_comments" IS 'Given by grader';

COMMENT ON COLUMN "status_updates"."title" IS 'Optional title. If not set, can use open_at date';

COMMENT ON COLUMN "status_updates"."open_date_time" IS 'When submissions start to be accepted';

COMMENT ON COLUMN "status_updates"."close_date_time" IS 'When submissions stop being submittable';

COMMENT ON COLUMN "meetings"."is_public" IS 'True if it appears on the schedule publicly (can be used for drafts)';

COMMENT ON COLUMN "meetings"."title" IS 'Optional meeting title';

COMMENT ON COLUMN "meetings"."agenda" IS 'List of agenda items';

COMMENT ON COLUMN "meetings"."location" IS 'Physical location or URL to join';

COMMENT ON COLUMN "meeting_attendances"."is_manually_added" IS 'True if manually added by admin and not user';

COMMENT ON TABLE "bonus_attendances" IS 'Bonus attendances from different events';

COMMENT ON TABLE "project_presentation_grades" IS 'Grades for end of semester project presentations. Might need to separate grade into multiple.';

COMMENT ON TABLE "chat_associations" IS 'Association of chat platform channel or other entity with a small group or project.';

COMMENT ON COLUMN "chat_associations"."source_id" IS 'ID of source, e.g. project id or small group id';

COMMENT ON COLUMN "chat_associations"."target_id" IS 'ID of target on platform, e.g. Discord channel ID';

COMMENT ON COLUMN "final_grade_appeal"."is_handled" IS 'Whether a faculty advisor has handled this appeal yet.';
