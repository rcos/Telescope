//! GraphQL query to check if a user can view draft projects.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::api::rcos::users::UserRole;
use crate::error::TelescopeError;
use chrono::Local;

/// Type representing GraphQL query to check if a user can view drafts.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/authorization_for.graphql"
)]
pub struct AuthorizationFor;

use authorization_for::{ResponseData, Variables};

/// Info on the user that dictates their ability to access project data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserProjectAuthorization {
    /// The user's ID.
    pub user_id: Option<uuid>,
    /// The user's role. Faculty advisors can access just about anything.
    role: UserRole,
    /// Is this user a coordinator during an ongoing semester?
    is_current_coordinator: bool,
    /// Is this user a mentor during an ongoing semester?
    is_current_mentor: bool,
}

impl Default for UserProjectAuthorization {
    fn default() -> Self {
        Self {
            user_id: None,
            role: UserRole::External,
            is_current_coordinator: false,
            is_current_mentor: false,
        }
    }
}

impl UserProjectAuthorization {
    /// Create an authorization object for a faculty advisor.
    fn faculty_advisor(user_id: uuid) -> Self {
        Self {
            user_id: Some(user_id),
            role: UserRole::FacultyAdvisor,
            is_current_mentor: false,
            is_current_coordinator: false,
        }
    }

    /// Is this user a coordinator or faculty?
    pub fn is_staff(&self) -> bool {
        self.is_current_coordinator
            || self.role == UserRole::FacultyAdvisor
            || self.role == UserRole::Sysadmin
    }

    /// Can the user associated with this authorization view projects of a given type?
    pub fn can_view(&self) -> bool {
        true
    }

    /// Can the user associated with this authorization edit projects with a given type
    /// and optionally specified host user ID?
    pub fn can_edit(&self) -> bool { //TODO allow the creator of the project to modify
            // of the viewer is a coordinator or faculty advisor
            self.is_staff()
    }


    /// Can the user associated with this authorization delete projects?
    /// This is currently just coordinators and faculty advisors.
    pub fn can_delete(&self) -> bool {
        self.is_staff()
    }

    /// Can the user associated with this authorization create projects?
    /// This is currently just coordinators and faculty advisors.
    pub fn can_create(&self) -> bool {
        self.is_staff()
    }

}

impl AuthorizationFor {
    /// Get the project access authorization rules for a given user.
    pub async fn get(user_id: Option<uuid>) -> Result<UserProjectAuthorization, TelescopeError> {
        // If there is no user ID, then the viewer has default (lowest) authorization.
        if user_id.is_none() {
            return Ok(UserProjectAuthorization::default());
        }

        // Otherwise unwrap the user ID.
        let user_id = user_id.unwrap();

        // Create variables for an API query.
        let query_vars: Variables = Variables {
            // Use the current local date.
            now: Local::today().naive_local(),
            user_id,
        };

        // Call the API.
        let api_response: ResponseData = send_query::<Self>(query_vars).await?;

        // First check if the user is a faculty advisor.
        let user_role: UserRole = api_response
            .users_by_pk
            .map(|user| user.role)
            .unwrap_or(UserRole::External);

        if user_role == UserRole::FacultyAdvisor {
            return Ok(UserProjectAuthorization::faculty_advisor(user_id));
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

        return Ok(UserProjectAuthorization {
            user_id: Some(user_id),
            role: user_role,
            is_current_coordinator,
            is_current_mentor,
        });
    }
}
