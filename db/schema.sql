SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: chat_association_source; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.chat_association_source AS ENUM (
    'project',
    'small_group'
);


--
-- Name: TYPE chat_association_source; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.chat_association_source IS 'The kind of group this chat is for';


--
-- Name: chat_association_target; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.chat_association_target AS ENUM (
    'discord_server',
    'discord_text_channel',
    'discord_voice_channel',
    'discord_category',
    'discord_role'
);


--
-- Name: TYPE chat_association_target; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.chat_association_target IS 'The kind of chat that this refers to';


--
-- Name: meeting_type; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.meeting_type AS ENUM (
    'large_group',
    'small_group',
    'presentations',
    'bonus_session',
    'grading',
    'mentors',
    'coordinators',
    'other'
);


--
-- Name: TYPE meeting_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.meeting_type IS 'The kind of RCOS meeting this was';


--
-- Name: url; Type: DOMAIN; Schema: public; Owner: -
--

CREATE DOMAIN public.url AS text
	CONSTRAINT url_check CHECK ((VALUE ~ 'https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#()?&//=]*)'::text));


--
-- Name: DOMAIN url; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON DOMAIN public.url IS 'Type that match URLs (http or https)';


--
-- Name: user_account; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.user_account AS ENUM (
    'rpi',
    'discord',
    'mattermost',
    'github',
    'gitlab',
    'bitbucket'
);


--
-- Name: TYPE user_account; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.user_account IS 'The website this account is for';


--
-- Name: user_role; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.user_role AS ENUM (
    'student',
    'faculty',
    'faculty_advisor',
    'alumn',
    'external',
    'external_mentor'
);


--
-- Name: TYPE user_role; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.user_role IS 'The user''s position within RCOS';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: announcements; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.announcements (
    announcement_id integer NOT NULL,
    semester_id character varying NOT NULL,
    title character varying NOT NULL,
    body_markdown text NOT NULL,
    close_date_time timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE announcements; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.announcements IS 'Various announcements  made by RCOS';


--
-- Name: COLUMN announcements.title; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.announcements.title IS 'Short title of announcement';


--
-- Name: COLUMN announcements.body_markdown; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.announcements.body_markdown IS 'Markdown-supported announcement content';


--
-- Name: COLUMN announcements.close_date_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.announcements.close_date_time IS 'Date and time the announcement ends';


--
-- Name: announcements_announcement_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.announcements_announcement_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: announcements_announcement_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.announcements_announcement_id_seq OWNED BY public.announcements.announcement_id;


--
-- Name: bonus_attendances; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.bonus_attendances (
    bonus_attendance_id integer NOT NULL,
    semester_id character varying NOT NULL,
    username character varying NOT NULL,
    reason text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE bonus_attendances; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.bonus_attendances IS 'Bonus attendances from different events';


--
-- Name: bonus_attendances_bonus_attendance_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.bonus_attendances_bonus_attendance_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: bonus_attendances_bonus_attendance_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.bonus_attendances_bonus_attendance_id_seq OWNED BY public.bonus_attendances.bonus_attendance_id;


--
-- Name: chat_associations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.chat_associations (
    source_type public.chat_association_source NOT NULL,
    target_type public.chat_association_target NOT NULL,
    source_id character varying NOT NULL,
    target_id character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE chat_associations; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.chat_associations IS 'Association of chat platform channel or
other entity with a small group or project';


--
-- Name: COLUMN chat_associations.source_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.chat_associations.source_type IS 'What the target is
associated with, e.g. project or small group';


--
-- Name: COLUMN chat_associations.target_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.chat_associations.target_type IS 'What the source is
associated with, e.g. Discord TEXT channel';


--
-- Name: COLUMN chat_associations.source_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.chat_associations.source_id IS 'ID of source, e.g. project id
or small group id';


--
-- Name: COLUMN chat_associations.target_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.chat_associations.target_id IS 'ID of target on platform, e.g.
Discord channel ID';


--
-- Name: enrollments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.enrollments (
    semester_id character varying NOT NULL,
    username character varying NOT NULL,
    project_id integer,
    is_project_lead boolean DEFAULT false,
    is_coordinator boolean DEFAULT false,
    credits integer DEFAULT 0 NOT NULL,
    is_for_pay boolean DEFAULT false,
    mid_year_grade real,
    final_grade real,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT enrollments_final_grade_check CHECK ((final_grade >= (0.0)::double precision)),
    CONSTRAINT enrollments_mid_year_grade_check CHECK ((mid_year_grade >= (0.0)::double precision))
);


--
-- Name: TABLE enrollments; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.enrollments IS 'An enrollment of a user in RCOS for a specific
semester. They might or might not be on a project and might or might not be
taking RCOS for credit.';


--
-- Name: COLUMN enrollments.is_project_lead; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.enrollments.is_project_lead IS 'Allows multiple project leads';


--
-- Name: COLUMN enrollments.credits; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.enrollments.credits IS '0-4 where 0 means just for experience';


--
-- Name: COLUMN enrollments.is_for_pay; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.enrollments.is_for_pay IS 'True if taking RCOS for pay';


--
-- Name: COLUMN enrollments.mid_year_grade; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.enrollments.mid_year_grade IS '0.0-100.0';


--
-- Name: COLUMN enrollments.final_grade; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.enrollments.final_grade IS '0.0-100.0';


--
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.users (
    username character varying NOT NULL,
    preferred_name character varying,
    first_name character varying NOT NULL,
    last_name character varying NOT NULL,
    cohort integer,
    role public.user_role NOT NULL,
    timezone text DEFAULT 'America/New_York'::text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE users; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.users IS 'Users can be students, external mentors, and faculty.
Their user details are not dependent on the semester';


--
-- Name: COLUMN users.preferred_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.preferred_name IS 'Optional preferred first name to use in UIs';


--
-- Name: COLUMN users.first_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.first_name IS 'Given name of user';


--
-- Name: COLUMN users.last_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.last_name IS 'Family name of user';


--
-- Name: COLUMN users.cohort; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.cohort IS 'Entry year (only set for students)';


--
-- Name: COLUMN users.role; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.role IS 'Role of user in RCOS, determines permissions';


--
-- Name: COLUMN users.timezone; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.timezone IS 'Timezone from TZ list';


--
-- Name: coordinators; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.coordinators AS
 SELECT DISTINCT e.semester_id,
    u.username,
    u.preferred_name,
    u.first_name,
    u.last_name
   FROM (public.users u
     JOIN public.enrollments e ON (((e.username)::text = (u.username)::text)))
  WHERE (e.is_coordinator = true)
  ORDER BY e.semester_id, u.username;


--
-- Name: VIEW coordinators; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON VIEW public.coordinators IS 'View for access to Coordinators each semester';


--
-- Name: faculty_advisors; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.faculty_advisors AS
 SELECT u.username,
    u.preferred_name,
    u.first_name,
    u.last_name
   FROM public.users u
  WHERE (u.role = 'faculty_advisor'::public.user_role);


--
-- Name: VIEW faculty_advisors; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON VIEW public.faculty_advisors IS 'View for access to Faculty Advisors';


--
-- Name: final_grade_appeal; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.final_grade_appeal (
    semester_id character varying NOT NULL,
    username character varying NOT NULL,
    expected_grade character varying NOT NULL,
    reason text NOT NULL,
    is_handled boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN final_grade_appeal.expected_grade; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.final_grade_appeal.expected_grade IS 'Grade the student
expected to receive';


--
-- Name: COLUMN final_grade_appeal.reason; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.final_grade_appeal.reason IS 'Reason the student believes they
deserve expected_grade';


--
-- Name: COLUMN final_grade_appeal.is_handled; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.final_grade_appeal.is_handled IS 'Whether a faculty advisor
has handled this appeal yet';


--
-- Name: meeting_attendances; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.meeting_attendances (
    meeting_id integer NOT NULL,
    username character varying NOT NULL,
    is_manually_added boolean DEFAULT false,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN meeting_attendances.is_manually_added; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.meeting_attendances.is_manually_added IS 'True if manually
added by admin and not user';


--
-- Name: meetings; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.meetings (
    meeting_id integer NOT NULL,
    semester_id character varying NOT NULL,
    type public.meeting_type NOT NULL,
    host_username character varying,
    is_public boolean DEFAULT true,
    start_date_time timestamp with time zone NOT NULL,
    end_date_time timestamp with time zone NOT NULL,
    title character varying,
    agenda character varying[] DEFAULT '{}'::character varying[],
    presentation_markdown text,
    external_presentation_url public.url,
    attendance_code character varying,
    recording_url public.url,
    is_remote boolean DEFAULT false NOT NULL,
    location character varying,
    meeting_url public.url,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT meetings_check CHECK (((is_remote AND (meeting_url IS NOT NULL)) OR ((NOT is_remote) AND (location IS NOT NULL))))
);


--
-- Name: COLUMN meetings.host_username; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.meetings.host_username IS 'Optional host of meeting, e.g.
mentor username for bonus workshop';


--
-- Name: COLUMN meetings.is_public; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.meetings.is_public IS 'True if it appears on the schedule
publicly (can be used for drafts)';


--
-- Name: COLUMN meetings.title; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.meetings.title IS 'Optional meeting title';


--
-- Name: COLUMN meetings.agenda; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.meetings.agenda IS 'List of agenda items that will be covered in the meeting';


--
-- Name: COLUMN meetings.presentation_markdown; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.meetings.presentation_markdown IS 'Markdown for a RevealJS
presentation that is used to auto-generate the presentation';


--
-- Name: COLUMN meetings.external_presentation_url; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.meetings.external_presentation_url IS 'Link to external
presentation if markdown generated one is not used';


--
-- Name: COLUMN meetings.location; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.meetings.location IS 'Physical location or URL to join';


--
-- Name: meetings_meeting_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.meetings_meeting_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: meetings_meeting_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.meetings_meeting_id_seq OWNED BY public.meetings.meeting_id;


--
-- Name: mentor_proposals; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.mentor_proposals (
    semester_id character varying NOT NULL,
    username character varying NOT NULL,
    reason text NOT NULL,
    skillset text NOT NULL,
    reviewer_username character varying,
    reviewer_comments text,
    is_approved boolean DEFAULT false,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE mentor_proposals; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.mentor_proposals IS 'Users Interested in mentoring each
semester must submit a proposal and be approved';


--
-- Name: COLUMN mentor_proposals.username; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mentor_proposals.username IS 'Username of mentor to-be';


--
-- Name: COLUMN mentor_proposals.reason; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mentor_proposals.reason IS 'The reason the user would like to mentor';


--
-- Name: COLUMN mentor_proposals.skillset; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mentor_proposals.skillset IS 'Short details of technologies
user can mentor for';


--
-- Name: COLUMN mentor_proposals.reviewer_username; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mentor_proposals.reviewer_username IS 'Username of
coordinator/faculty who reviewed proposal';


--
-- Name: COLUMN mentor_proposals.reviewer_comments; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mentor_proposals.reviewer_comments IS 'Optional comments left by reviewer';


--
-- Name: COLUMN mentor_proposals.is_approved; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mentor_proposals.is_approved IS 'True if user was approved to
become a mentor for the semester';


--
-- Name: pay_requests; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.pay_requests (
    semester_id character varying NOT NULL,
    username character varying NOT NULL,
    reason text NOT NULL,
    is_approved boolean DEFAULT false,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE pay_requests; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.pay_requests IS 'Users can request to take RCOS for pay INSTEAD
of credit and must be approved';


--
-- Name: COLUMN pay_requests.reason; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.pay_requests.reason IS 'The justification for being paid';


--
-- Name: project_pitches; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.project_pitches (
    semester_id character varying NOT NULL,
    username character varying NOT NULL,
    existing_project_id integer,
    proposed_title character varying,
    proposed_description text,
    proposed_stack text,
    pitch_slide_url public.url,
    proposal_url public.url,
    is_looking_for_members boolean DEFAULT true NOT NULL,
    is_approved boolean DEFAULT false NOT NULL,
    reviewer_username character varying,
    reviewer_comments text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE project_pitches; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.project_pitches IS 'Represents a project pitch by a member at
the start of a semester.  If the pitch is for an existing project, the title,
description, stack can be grabbed. Otherwise, when the proposal is approved
those fields are used to create the actual project';


--
-- Name: COLUMN project_pitches.existing_project_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.project_pitches.existing_project_id IS 'Only if pitch for
existing RCOS project';


--
-- Name: COLUMN project_pitches.proposed_title; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.project_pitches.proposed_title IS 'Null if for existing RCOS project';


--
-- Name: COLUMN project_pitches.pitch_slide_url; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.project_pitches.pitch_slide_url IS 'Link to 1-slide
presentation for pitch (if they are open)';


--
-- Name: COLUMN project_pitches.proposal_url; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.project_pitches.proposal_url IS 'Link to semester project proposal';


--
-- Name: COLUMN project_pitches.is_looking_for_members; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.project_pitches.is_looking_for_members IS 'Open to new members?';


--
-- Name: COLUMN project_pitches.reviewer_comments; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.project_pitches.reviewer_comments IS 'Optional notes from graders';


--
-- Name: project_presentation_grades; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.project_presentation_grades (
    semester_id character varying NOT NULL,
    project_id integer NOT NULL,
    grader_username character varying NOT NULL,
    grade real NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT project_presentation_grades_grade_check CHECK ((grade >= (0.0)::double precision))
);


--
-- Name: TABLE project_presentation_grades; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.project_presentation_grades IS 'Grades for end of semester
project presentations. Might need to separate grade Into multiple';


--
-- Name: project_presentations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.project_presentations (
    project_id integer NOT NULL,
    semester_id character varying NOT NULL,
    presentation_url public.url NOT NULL,
    is_draft boolean DEFAULT true NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE project_presentations; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.project_presentations IS 'Presentations given by RCOS projects';


--
-- Name: projects; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.projects (
    project_id integer NOT NULL,
    title character varying NOT NULL,
    description text NOT NULL,
    languages character varying[] DEFAULT '{}'::character varying[] NOT NULL,
    stack character varying[] DEFAULT '{}'::character varying[] NOT NULL,
    cover_image_url public.url,
    homepage_url public.url,
    repository_urls public.url[] NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE projects; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.projects IS 'Project details are not semester dependent';


--
-- Name: COLUMN projects.languages; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.languages IS 'List of languages used, all lowercase';


--
-- Name: COLUMN projects.stack; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.stack IS 'List of technologies used';


--
-- Name: COLUMN projects.cover_image_url; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.cover_image_url IS 'URL to logo image';


--
-- Name: COLUMN projects.homepage_url; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.homepage_url IS 'Optional link to project homepage';


--
-- Name: projects_project_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.projects_project_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: projects_project_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.projects_project_id_seq OWNED BY public.projects.project_id;


--
-- Name: public_meetings; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.public_meetings AS
 SELECT m.meeting_id,
    m.semester_id,
    m.type,
    m.host_username,
    m.is_public,
    m.start_date_time,
    m.end_date_time,
    m.title,
    m.agenda,
    m.presentation_markdown,
    m.external_presentation_url,
    m.attendance_code,
    m.recording_url,
    m.is_remote,
    m.location,
    m.meeting_url,
    m.created_at
   FROM public.meetings m
  WHERE (m.is_public = true)
  ORDER BY m.start_date_time;


--
-- Name: VIEW public_meetings; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON VIEW public.public_meetings IS 'View for access to public meetings';


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schema_migrations (
    version character varying(255) NOT NULL
);


--
-- Name: semesters; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.semesters (
    semester_id character varying(6) NOT NULL,
    title character varying NOT NULL,
    start_date date NOT NULL,
    end_date date NOT NULL,
    CONSTRAINT semesters_check CHECK ((end_date > start_date)),
    CONSTRAINT semesters_semester_id_check CHECK (((semester_id)::text ~ '^\d{6}$'::text))
);


--
-- Name: TABLE semesters; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.semesters IS 'Dates are from official academic calendar:
https://info.rpi.edu/registrar/academic-calendar
A school year has 3 semesters, Spring, Summer, and Fall. Semester IDs are
4-digit starting year + 2-digit start month, e.g. 202009';


--
-- Name: COLUMN semesters.title; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.semesters.title IS 'Typically season and year, e.g. Fall 2020';


--
-- Name: COLUMN semesters.start_date; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.semesters.start_date IS 'Date that classes start';


--
-- Name: COLUMN semesters.end_date; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.semesters.end_date IS 'Date that semester ends';


--
-- Name: small_group_projects; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.small_group_projects (
    small_group_id integer NOT NULL,
    project_id integer NOT NULL
);


--
-- Name: TABLE small_group_projects; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.small_group_projects IS 'Relation between small groups and
projects';


--
-- Name: small_groups; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.small_groups (
    small_group_id integer NOT NULL,
    semester_id character varying NOT NULL,
    title character varying NOT NULL,
    location character varying
);


--
-- Name: TABLE small_groups; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.small_groups IS 'A small group for a specific semester. There
will likely be repeats over semesters only differentiated by semester id.';


--
-- Name: COLUMN small_groups.title; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.small_groups.title IS 'The title of the small group.';


--
-- Name: COLUMN small_groups.location; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.small_groups.location IS 'Possible physical location of small
group, i.e. building and room';


--
-- Name: small_group_members; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.small_group_members AS
 SELECT sg.small_group_id,
    e.semester_id,
    e.username,
    e.project_id,
    e.is_project_lead,
    e.is_coordinator,
    e.credits,
    e.is_for_pay,
    e.mid_year_grade,
    e.final_grade,
    e.created_at
   FROM (((public.enrollments e
     JOIN public.projects p ON ((p.project_id = e.project_id)))
     JOIN public.small_group_projects sgp ON ((sgp.project_id = p.project_id)))
     JOIN public.small_groups sg ON ((sg.small_group_id = sgp.small_group_id)));


--
-- Name: VIEW small_group_members; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON VIEW public.small_group_members IS 'View for easy access to small group members';


--
-- Name: small_group_mentors; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.small_group_mentors (
    small_group_id integer NOT NULL,
    username character varying NOT NULL
);


--
-- Name: TABLE small_group_mentors; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.small_group_mentors IS 'Relation between small groups and
users who are the group''s mentors';


--
-- Name: small_groups_small_group_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.small_groups_small_group_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: small_groups_small_group_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.small_groups_small_group_id_seq OWNED BY public.small_groups.small_group_id;


--
-- Name: status_update_submissions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.status_update_submissions (
    status_update_id integer NOT NULL,
    username character varying NOT NULL,
    this_week text NOT NULL,
    next_week text NOT NULL,
    blockers text NOT NULL,
    grade real,
    grader_username character varying,
    grader_comments text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT status_update_submissions_grade_check CHECK ((grade >= (0.0)::double precision))
);


--
-- Name: TABLE status_update_submissions; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.status_update_submissions IS 'A status update submission by a enrolled member';


--
-- Name: COLUMN status_update_submissions.grade; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.status_update_submissions.grade IS 'Scale from 0-1: did this
status update meet the requirements';


--
-- Name: COLUMN status_update_submissions.grader_username; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.status_update_submissions.grader_username IS 'The
mentor/coordinator/faculty member that graded this status_update';


--
-- Name: COLUMN status_update_submissions.grader_comments; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.status_update_submissions.grader_comments IS 'Given by grader';


--
-- Name: status_updates; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.status_updates (
    status_update_id integer NOT NULL,
    semester_id character varying NOT NULL,
    title character varying,
    open_date_time timestamp with time zone NOT NULL,
    close_date_time timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT status_updates_check CHECK (((close_date_time IS NULL) OR (close_date_time > open_date_time)))
);


--
-- Name: COLUMN status_updates.title; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.status_updates.title IS 'Optional title. If not set, can use open_at date';


--
-- Name: COLUMN status_updates.open_date_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.status_updates.open_date_time IS 'When submissions start to be accepted';


--
-- Name: COLUMN status_updates.close_date_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.status_updates.close_date_time IS 'When submissions stop being submittable';


--
-- Name: status_updates_status_update_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.status_updates_status_update_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: status_updates_status_update_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.status_updates_status_update_id_seq OWNED BY public.status_updates.status_update_id;


--
-- Name: user_accounts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_accounts (
    username character varying NOT NULL,
    type public.user_account NOT NULL,
    account_id character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE user_accounts; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.user_accounts IS 'User accounts such as Discord, GitHub, GitLab, etc.';


--
-- Name: COLUMN user_accounts.type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_accounts.type IS 'Type of external account that is connected';


--
-- Name: COLUMN user_accounts.account_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_accounts.account_id IS 'Unique ID/username of account';


--
-- Name: workshop_proposals; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.workshop_proposals (
    workshop_proposal_id integer NOT NULL,
    semester_id character varying NOT NULL,
    username character varying NOT NULL,
    topic character varying NOT NULL,
    title character varying NOT NULL,
    qualifications character varying NOT NULL,
    first_choice_at timestamp with time zone NOT NULL,
    second_choice_at timestamp with time zone NOT NULL,
    third_choice_at timestamp with time zone NOT NULL,
    reviewer_username character varying,
    reviewer_comments text,
    is_approved boolean DEFAULT false,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT workshop_proposals_first_choice_at_check CHECK ((first_choice_at > now())),
    CONSTRAINT workshop_proposals_second_choice_at_check CHECK ((second_choice_at > now())),
    CONSTRAINT workshop_proposals_third_choice_at_check CHECK ((third_choice_at > now()))
);


--
-- Name: TABLE workshop_proposals; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.workshop_proposals IS 'Users (typically mentors) must submit a
proposal to host a workshop and be approved';


--
-- Name: COLUMN workshop_proposals.first_choice_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.workshop_proposals.first_choice_at IS 'First choice for date
and time to host workshop';


--
-- Name: COLUMN workshop_proposals.second_choice_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.workshop_proposals.second_choice_at IS 'Second choice for date
and time to host workshop';


--
-- Name: COLUMN workshop_proposals.third_choice_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.workshop_proposals.third_choice_at IS 'Third choice for date
and time to host workshop';


--
-- Name: COLUMN workshop_proposals.reviewer_username; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.workshop_proposals.reviewer_username IS 'Username of
coordinator/faculty who reviewed proposal';


--
-- Name: COLUMN workshop_proposals.reviewer_comments; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.workshop_proposals.reviewer_comments IS 'Optional comments left by reviewer';


--
-- Name: workshop_proposals_workshop_proposal_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.workshop_proposals_workshop_proposal_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: workshop_proposals_workshop_proposal_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.workshop_proposals_workshop_proposal_id_seq OWNED BY public.workshop_proposals.workshop_proposal_id;


--
-- Name: announcements announcement_id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.announcements ALTER COLUMN announcement_id SET DEFAULT nextval('public.announcements_announcement_id_seq'::regclass);


--
-- Name: bonus_attendances bonus_attendance_id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.bonus_attendances ALTER COLUMN bonus_attendance_id SET DEFAULT nextval('public.bonus_attendances_bonus_attendance_id_seq'::regclass);


--
-- Name: meetings meeting_id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.meetings ALTER COLUMN meeting_id SET DEFAULT nextval('public.meetings_meeting_id_seq'::regclass);


--
-- Name: projects project_id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.projects ALTER COLUMN project_id SET DEFAULT nextval('public.projects_project_id_seq'::regclass);


--
-- Name: small_groups small_group_id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.small_groups ALTER COLUMN small_group_id SET DEFAULT nextval('public.small_groups_small_group_id_seq'::regclass);


--
-- Name: status_updates status_update_id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.status_updates ALTER COLUMN status_update_id SET DEFAULT nextval('public.status_updates_status_update_id_seq'::regclass);


--
-- Name: workshop_proposals workshop_proposal_id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workshop_proposals ALTER COLUMN workshop_proposal_id SET DEFAULT nextval('public.workshop_proposals_workshop_proposal_id_seq'::regclass);


--
-- Name: announcements announcements_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.announcements
    ADD CONSTRAINT announcements_pkey PRIMARY KEY (announcement_id);


--
-- Name: bonus_attendances bonus_attendances_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.bonus_attendances
    ADD CONSTRAINT bonus_attendances_pkey PRIMARY KEY (bonus_attendance_id);


--
-- Name: chat_associations chat_associations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.chat_associations
    ADD CONSTRAINT chat_associations_pkey PRIMARY KEY (source_type, target_type, source_id);


--
-- Name: enrollments enrollments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.enrollments
    ADD CONSTRAINT enrollments_pkey PRIMARY KEY (semester_id, username);


--
-- Name: final_grade_appeal final_grade_appeal_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.final_grade_appeal
    ADD CONSTRAINT final_grade_appeal_pkey PRIMARY KEY (semester_id, username);


--
-- Name: meeting_attendances meeting_attendances_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.meeting_attendances
    ADD CONSTRAINT meeting_attendances_pkey PRIMARY KEY (meeting_id, username);


--
-- Name: meetings meetings_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.meetings
    ADD CONSTRAINT meetings_pkey PRIMARY KEY (meeting_id);


--
-- Name: mentor_proposals mentor_proposals_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mentor_proposals
    ADD CONSTRAINT mentor_proposals_pkey PRIMARY KEY (semester_id, username);


--
-- Name: pay_requests pay_requests_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.pay_requests
    ADD CONSTRAINT pay_requests_pkey PRIMARY KEY (semester_id, username);


--
-- Name: project_pitches project_pitches_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_pitches
    ADD CONSTRAINT project_pitches_pkey PRIMARY KEY (semester_id, username);


--
-- Name: project_presentation_grades project_presentation_grades_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_presentation_grades
    ADD CONSTRAINT project_presentation_grades_pkey PRIMARY KEY (semester_id, project_id, grader_username);


--
-- Name: project_presentations project_presentations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_presentations
    ADD CONSTRAINT project_presentations_pkey PRIMARY KEY (project_id, semester_id);


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (project_id);


--
-- Name: projects projects_title_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_title_key UNIQUE (title);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: semesters semesters_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.semesters
    ADD CONSTRAINT semesters_pkey PRIMARY KEY (semester_id);


--
-- Name: small_group_mentors small_group_mentors_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.small_group_mentors
    ADD CONSTRAINT small_group_mentors_pkey PRIMARY KEY (small_group_id, username);


--
-- Name: small_group_projects small_group_projects_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.small_group_projects
    ADD CONSTRAINT small_group_projects_pkey PRIMARY KEY (small_group_id, project_id);


--
-- Name: small_groups small_groups_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.small_groups
    ADD CONSTRAINT small_groups_pkey PRIMARY KEY (small_group_id);


--
-- Name: status_update_submissions status_update_submissions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.status_update_submissions
    ADD CONSTRAINT status_update_submissions_pkey PRIMARY KEY (status_update_id, username);


--
-- Name: status_updates status_updates_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.status_updates
    ADD CONSTRAINT status_updates_pkey PRIMARY KEY (status_update_id);


--
-- Name: user_accounts user_accounts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_accounts
    ADD CONSTRAINT user_accounts_pkey PRIMARY KEY (username, type);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (username);


--
-- Name: workshop_proposals workshop_proposals_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workshop_proposals
    ADD CONSTRAINT workshop_proposals_pkey PRIMARY KEY (workshop_proposal_id);


--
-- Name: bonus_attendances_semester_id_username_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX bonus_attendances_semester_id_username_idx ON public.bonus_attendances USING btree (semester_id, username);


--
-- Name: enrollments_credits_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX enrollments_credits_idx ON public.enrollments USING btree (credits) WHERE (credits > 0);


--
-- Name: enrollments_project_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX enrollments_project_id_idx ON public.enrollments USING btree (project_id);


--
-- Name: meetings_is_public_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX meetings_is_public_idx ON public.meetings USING btree (is_public);


--
-- Name: meetings_semester_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX meetings_semester_id_idx ON public.meetings USING btree (semester_id);


--
-- Name: meetings_start_date_time_end_date_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX meetings_start_date_time_end_date_time_idx ON public.meetings USING btree (start_date_time, end_date_time);


--
-- Name: meetings_type_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX meetings_type_idx ON public.meetings USING btree (type);


--
-- Name: semesters_start_date_end_date_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX semesters_start_date_end_date_idx ON public.semesters USING btree (start_date, end_date);


--
-- Name: small_groups_semester_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX small_groups_semester_id_idx ON public.small_groups USING btree (semester_id);


--
-- Name: small_groups_semester_id_title_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX small_groups_semester_id_title_idx ON public.small_groups USING btree (semester_id, title);


--
-- Name: workshop_proposals_semester_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX workshop_proposals_semester_id_idx ON public.workshop_proposals USING btree (semester_id);


--
-- Name: workshop_proposals_username_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX workshop_proposals_username_idx ON public.workshop_proposals USING btree (username);


--
-- Name: announcements announcements_semester_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.announcements
    ADD CONSTRAINT announcements_semester_id_fkey FOREIGN KEY (semester_id) REFERENCES public.semesters(semester_id);


--
-- Name: bonus_attendances bonus_attendances_semester_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.bonus_attendances
    ADD CONSTRAINT bonus_attendances_semester_id_fkey FOREIGN KEY (semester_id) REFERENCES public.semesters(semester_id);


--
-- Name: bonus_attendances bonus_attendances_semester_id_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.bonus_attendances
    ADD CONSTRAINT bonus_attendances_semester_id_username_fkey FOREIGN KEY (semester_id, username) REFERENCES public.enrollments(semester_id, username);


--
-- Name: bonus_attendances bonus_attendances_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.bonus_attendances
    ADD CONSTRAINT bonus_attendances_username_fkey FOREIGN KEY (username) REFERENCES public.users(username);


--
-- Name: enrollments enrollments_project_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.enrollments
    ADD CONSTRAINT enrollments_project_id_fkey FOREIGN KEY (project_id) REFERENCES public.projects(project_id);


--
-- Name: enrollments enrollments_semester_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.enrollments
    ADD CONSTRAINT enrollments_semester_id_fkey FOREIGN KEY (semester_id) REFERENCES public.semesters(semester_id);


--
-- Name: enrollments enrollments_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.enrollments
    ADD CONSTRAINT enrollments_username_fkey FOREIGN KEY (username) REFERENCES public.users(username);


--
-- Name: final_grade_appeal final_grade_appeal_semester_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.final_grade_appeal
    ADD CONSTRAINT final_grade_appeal_semester_id_fkey FOREIGN KEY (semester_id) REFERENCES public.semesters(semester_id);


--
-- Name: final_grade_appeal final_grade_appeal_semester_id_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.final_grade_appeal
    ADD CONSTRAINT final_grade_appeal_semester_id_username_fkey FOREIGN KEY (semester_id, username) REFERENCES public.enrollments(semester_id, username);


--
-- Name: final_grade_appeal final_grade_appeal_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.final_grade_appeal
    ADD CONSTRAINT final_grade_appeal_username_fkey FOREIGN KEY (username) REFERENCES public.users(username);


--
-- Name: meeting_attendances meeting_attendances_meeting_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.meeting_attendances
    ADD CONSTRAINT meeting_attendances_meeting_id_fkey FOREIGN KEY (meeting_id) REFERENCES public.meetings(meeting_id);


--
-- Name: meeting_attendances meeting_attendances_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.meeting_attendances
    ADD CONSTRAINT meeting_attendances_username_fkey FOREIGN KEY (username) REFERENCES public.users(username);


--
-- Name: meetings meetings_host_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.meetings
    ADD CONSTRAINT meetings_host_username_fkey FOREIGN KEY (host_username) REFERENCES public.users(username);


--
-- Name: meetings meetings_semester_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.meetings
    ADD CONSTRAINT meetings_semester_id_fkey FOREIGN KEY (semester_id) REFERENCES public.semesters(semester_id);


--
-- Name: meetings meetings_semester_id_host_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.meetings
    ADD CONSTRAINT meetings_semester_id_host_username_fkey FOREIGN KEY (semester_id, host_username) REFERENCES public.enrollments(semester_id, username);


--
-- Name: mentor_proposals mentor_proposals_semester_id_reviewer_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mentor_proposals
    ADD CONSTRAINT mentor_proposals_semester_id_reviewer_username_fkey FOREIGN KEY (semester_id, reviewer_username) REFERENCES public.enrollments(semester_id, username);


--
-- Name: mentor_proposals mentor_proposals_semester_id_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mentor_proposals
    ADD CONSTRAINT mentor_proposals_semester_id_username_fkey FOREIGN KEY (semester_id, username) REFERENCES public.enrollments(semester_id, username);


--
-- Name: pay_requests pay_requests_semester_id_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.pay_requests
    ADD CONSTRAINT pay_requests_semester_id_username_fkey FOREIGN KEY (semester_id, username) REFERENCES public.enrollments(semester_id, username);


--
-- Name: project_pitches project_pitches_existing_project_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_pitches
    ADD CONSTRAINT project_pitches_existing_project_id_fkey FOREIGN KEY (existing_project_id) REFERENCES public.projects(project_id);


--
-- Name: project_pitches project_pitches_reviewer_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_pitches
    ADD CONSTRAINT project_pitches_reviewer_username_fkey FOREIGN KEY (reviewer_username) REFERENCES public.users(username);


--
-- Name: project_pitches project_pitches_semester_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_pitches
    ADD CONSTRAINT project_pitches_semester_id_fkey FOREIGN KEY (semester_id) REFERENCES public.semesters(semester_id);


--
-- Name: project_pitches project_pitches_semester_id_reviewer_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_pitches
    ADD CONSTRAINT project_pitches_semester_id_reviewer_username_fkey FOREIGN KEY (semester_id, reviewer_username) REFERENCES public.enrollments(semester_id, username);


--
-- Name: project_pitches project_pitches_semester_id_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_pitches
    ADD CONSTRAINT project_pitches_semester_id_username_fkey FOREIGN KEY (semester_id, username) REFERENCES public.enrollments(semester_id, username);


--
-- Name: project_pitches project_pitches_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_pitches
    ADD CONSTRAINT project_pitches_username_fkey FOREIGN KEY (username) REFERENCES public.users(username);


--
-- Name: project_presentation_grades project_presentation_grades_grader_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_presentation_grades
    ADD CONSTRAINT project_presentation_grades_grader_username_fkey FOREIGN KEY (grader_username) REFERENCES public.users(username);


--
-- Name: project_presentation_grades project_presentation_grades_project_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_presentation_grades
    ADD CONSTRAINT project_presentation_grades_project_id_fkey FOREIGN KEY (project_id) REFERENCES public.projects(project_id);


--
-- Name: project_presentation_grades project_presentation_grades_semester_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_presentation_grades
    ADD CONSTRAINT project_presentation_grades_semester_id_fkey FOREIGN KEY (semester_id) REFERENCES public.semesters(semester_id);


--
-- Name: project_presentation_grades project_presentation_grades_semester_id_grader_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_presentation_grades
    ADD CONSTRAINT project_presentation_grades_semester_id_grader_username_fkey FOREIGN KEY (semester_id, grader_username) REFERENCES public.enrollments(semester_id, username);


--
-- Name: project_presentations project_presentations_project_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_presentations
    ADD CONSTRAINT project_presentations_project_id_fkey FOREIGN KEY (project_id) REFERENCES public.projects(project_id);


--
-- Name: project_presentations project_presentations_semester_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.project_presentations
    ADD CONSTRAINT project_presentations_semester_id_fkey FOREIGN KEY (semester_id) REFERENCES public.semesters(semester_id);


--
-- Name: small_group_mentors small_group_mentors_small_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.small_group_mentors
    ADD CONSTRAINT small_group_mentors_small_group_id_fkey FOREIGN KEY (small_group_id) REFERENCES public.small_groups(small_group_id);


--
-- Name: small_group_mentors small_group_mentors_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.small_group_mentors
    ADD CONSTRAINT small_group_mentors_username_fkey FOREIGN KEY (username) REFERENCES public.users(username);


--
-- Name: small_group_projects small_group_projects_project_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.small_group_projects
    ADD CONSTRAINT small_group_projects_project_id_fkey FOREIGN KEY (project_id) REFERENCES public.projects(project_id);


--
-- Name: small_group_projects small_group_projects_small_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.small_group_projects
    ADD CONSTRAINT small_group_projects_small_group_id_fkey FOREIGN KEY (small_group_id) REFERENCES public.small_groups(small_group_id);


--
-- Name: small_groups small_groups_semester_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.small_groups
    ADD CONSTRAINT small_groups_semester_id_fkey FOREIGN KEY (semester_id) REFERENCES public.semesters(semester_id);


--
-- Name: status_update_submissions status_update_submissions_grader_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.status_update_submissions
    ADD CONSTRAINT status_update_submissions_grader_username_fkey FOREIGN KEY (grader_username) REFERENCES public.users(username);


--
-- Name: status_update_submissions status_update_submissions_status_update_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.status_update_submissions
    ADD CONSTRAINT status_update_submissions_status_update_id_fkey FOREIGN KEY (status_update_id) REFERENCES public.status_updates(status_update_id);


--
-- Name: status_update_submissions status_update_submissions_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.status_update_submissions
    ADD CONSTRAINT status_update_submissions_username_fkey FOREIGN KEY (username) REFERENCES public.users(username);


--
-- Name: status_updates status_updates_semester_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.status_updates
    ADD CONSTRAINT status_updates_semester_id_fkey FOREIGN KEY (semester_id) REFERENCES public.semesters(semester_id);


--
-- Name: user_accounts user_accounts_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_accounts
    ADD CONSTRAINT user_accounts_username_fkey FOREIGN KEY (username) REFERENCES public.users(username);


--
-- Name: workshop_proposals workshop_proposals_reviewer_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workshop_proposals
    ADD CONSTRAINT workshop_proposals_reviewer_username_fkey FOREIGN KEY (reviewer_username) REFERENCES public.users(username);


--
-- Name: workshop_proposals workshop_proposals_semester_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workshop_proposals
    ADD CONSTRAINT workshop_proposals_semester_id_fkey FOREIGN KEY (semester_id) REFERENCES public.semesters(semester_id);


--
-- Name: workshop_proposals workshop_proposals_semester_id_reviewer_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workshop_proposals
    ADD CONSTRAINT workshop_proposals_semester_id_reviewer_username_fkey FOREIGN KEY (semester_id, reviewer_username) REFERENCES public.enrollments(semester_id, username);


--
-- Name: workshop_proposals workshop_proposals_semester_id_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workshop_proposals
    ADD CONSTRAINT workshop_proposals_semester_id_username_fkey FOREIGN KEY (semester_id, username) REFERENCES public.enrollments(semester_id, username);


--
-- Name: workshop_proposals workshop_proposals_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workshop_proposals
    ADD CONSTRAINT workshop_proposals_username_fkey FOREIGN KEY (username) REFERENCES public.users(username);


--
-- PostgreSQL database dump complete
--


--
-- Dbmate schema migrations
--

INSERT INTO public.schema_migrations (version) VALUES
    ('20210117171055'),
    ('20210117171712'),
    ('20210117172640'),
    ('20210117180501'),
    ('20210117180543'),
    ('20210117180819'),
    ('20210117182303'),
    ('20210117182615'),
    ('20210117183105'),
    ('20210117183106'),
    ('20210117183107'),
    ('20210117183108'),
    ('20210117183109'),
    ('20210117183357'),
    ('20210117183550'),
    ('20210117183837'),
    ('20210117184040'),
    ('20210117184142'),
    ('20210117184930'),
    ('20210117185159'),
    ('20210117185355'),
    ('20210117185840'),
    ('20210117190056'),
    ('20210117190201'),
    ('20210117190635'),
    ('20210117190735'),
    ('20210117191043'),
    ('20210117191050'),
    ('20210117194733');
