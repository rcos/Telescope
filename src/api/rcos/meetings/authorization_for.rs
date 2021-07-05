//! GraphQL query to check if a user can view draft meetings.

use crate::api::rcos::meetings::{MeetingType, ALL_MEETING_TYPES};
use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::api::rcos::users::UserRole;
use crate::error::TelescopeError;
use chrono::Local;

/// Type representing GraphQL query to check if a user can view drafts.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/authorization_for.graphql"
)]
pub struct AuthorizationFor;

use authorization_for::{ResponseData, Variables};
use crate::api::rcos::meetings::get_host::MeetingHost;

/// Info on the user that dictates their ability to access meeting data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserMeetingAuthorization {
    /// The user's username.
    pub username: Option<String>,
    /// The user's role. Faculty advisors can access just about anything.
    role: UserRole,
    /// Is this user a coordinator during an ongoing semester?
    is_current_coordinator: bool,
    /// Is this user a mentor during an ongoing semester?
    is_current_mentor: bool,
}

impl Default for UserMeetingAuthorization {
    fn default() -> Self {
        UserMeetingAuthorization {
            username: None,
            role: UserRole::External,
            is_current_coordinator: false,
            is_current_mentor: false,
        }
    }
}

impl UserMeetingAuthorization {
    /// Create an authorization object for a faculty advisor.
    fn faculty_advisor(username: String) -> Self {
        Self {
            username: Some(username),
            role: UserRole::FacultyAdvisor,
            is_current_mentor: false,
            is_current_coordinator: false,
        }
    }

    /// Can the user associated with this authorization view draft meetings?
    pub fn can_view_drafts(&self) -> bool {
        self.is_current_coordinator
            || self.role == UserRole::FacultyAdvisor
            || self.role == UserRole::Sysadmin
    }

    /// Can the user associated with this authorization view meetings of a given type?
    pub fn can_view(&self, meeting_type: MeetingType) -> bool {
        match meeting_type {
            // Coordinator meetings can be viewed by just coordinators and faculty advisors
            MeetingType::Coordinators => self.can_view_drafts(),

            // Mentor and Grading meetings can be viewed by mentors, coordinators,
            // and faculty advisors
            MeetingType::Mentors | MeetingType::Grading => {
                self.is_current_mentor || self.can_view_drafts()
            }

            // All other meeting types (small groups, large groups, bonus sessions, etc)
            // are public.
            _ => true,
        }
    }

    /// Can the user associated with this authorization edit meetings with a given type
    /// and optionally specified host username?
    pub fn can_edit(&self, host_username: Option<&str>) -> bool {
        // If there is a host and viewer
        if let (Some(host), Some(viewer)) = (host_username, self.username.as_ref()) {
            // and they are the same person (or the viewer has coordinator or higher perms)
            host == viewer || self.can_view_drafts()
        } else {
            // of the viewer is a coordinator or faculty advisor
            self.can_view_drafts()
        }
    }

    /// Can the user associated with this authorization edit the meeting
    pub async fn can_edit_by_id(&self, meeting_id: i64) -> Result<bool, TelescopeError> {
        // If the authenticated user is a coordinator or professor, then they can edit this meeting.
        if self.can_view_drafts() {
            Ok(true)
        } else {
            // Otherwise lookup the meeting and check if the authenticated username matches the host
            // username.
            let meeting_host: Option<String> = MeetingHost::get(meeting_id).await?;
            match (meeting_host, self.username.as_ref()) {
                // If there is both a host and a viewer, and they're the same,
                // the meeting can be edited.
                (Some(host), Some(viewer)) => Ok(host == *viewer),

                // In any other case, the meeting is not to be edited by the viewer.
                _ => Ok(false)
            }
        }
    }

    /// Can the user associated with this authorization delete meetings?
    /// This is currently just coordinators and faculty advisors.
    pub fn can_delete_meetings(&self) -> bool {
        self.can_view_drafts()
    }

    /// Can the user associated with this authorization create meetings?
    /// This is currently just coordinators and faculty advisors.
    pub fn can_create_meetings(&self) -> bool {
        self.can_view_drafts()
    }

    /// Get a list of the types of meetings viewable under this authorization.
    pub fn viewable_types(&self) -> Vec<MeetingType> {
        // Start with a vector of sufficient capacity to hold a full access list.
        let mut visible_types: Vec<MeetingType> = Vec::with_capacity(ALL_MEETING_TYPES.len());
        // Add all the types this user is authorized for.
        for ty in ALL_MEETING_TYPES.iter() {
            if self.can_view(*ty) {
                visible_types.push(*ty);
            }
        }
        return visible_types;
    }
}

impl AuthorizationFor {
    /// Get the meeting access authorization rules for a given user.
    pub async fn get(username: Option<String>) -> Result<UserMeetingAuthorization, TelescopeError> {
        // If there is no username, then the viewer has default (lowest) authorization.
        if username.is_none() {
            return Ok(UserMeetingAuthorization::default());
        }

        // Otherwise unwrap the username.
        let username: String = username.unwrap();

        // Create variables for an API query.
        let query_vars: Variables = Variables {
            // Use the current local date.
            now: Local::today().naive_local(),
            // Clone the username
            username: username.clone(),
        };

        // Call the API.
        let api_response: ResponseData = send_query::<Self>(query_vars).await?;

        // First check if the user is a faculty advisor.
        let user_role: UserRole = api_response
            .users_by_pk
            .map(|user| user.role)
            .unwrap_or(UserRole::External);

        if user_role == UserRole::FacultyAdvisor {
            return Ok(UserMeetingAuthorization::faculty_advisor(username));
        }

        // If they are not a faculty advisor, check if they are a current coordinator.
        let is_current_coordinator: bool = api_response
            // Start by flattening all the current semesters into one list of coordinator flags.
            .current_semesters
            .iter()
            .map(|semester| semester.enrollments.as_slice())
            .flatten()
            .map(|enrollment| enrollment.is_coordinator)
            // And check if any of them are true
            .any(|is_coordinator| is_coordinator);

        let is_current_mentor: bool = api_response
            // Flatten the current semesters' small groups where this user is a mentor.
            .current_semesters
            .iter()
            .map(|semester| semester.small_groups.as_slice())
            .flatten()
            .map(|small_group| small_group.small_group_id)
            // This user must be a mentor for at least one to be considered a current mentor.
            .count()
            >= 1;

        return Ok(UserMeetingAuthorization {
            username: Some(username),
            role: user_role,
            is_current_coordinator,
            is_current_mentor,
        });
    }
}
