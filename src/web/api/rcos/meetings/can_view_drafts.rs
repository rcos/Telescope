//! GraphQL query to check if a user can view draft meetings.

use crate::web::api::rcos::prelude::*;
use crate::error::TelescopeError;
use crate::web::api::rcos::send_query;
use chrono::Local;
use crate::web::api::rcos::users::UserRole;

/// Type representing GraphQL query to check if a user can view drafts.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/can_view_drafts.graphql",
)]
pub struct UserCanViewDrafts;

use user_can_view_drafts::{
    Variables, ResponseData
};

impl UserCanViewDrafts {
    /// Check if a user can view drafts by their identity. If username is `None` the response
    /// will always be false.
    pub async fn check(username: Option<String>) -> Result<bool, TelescopeError> {
        // If there is no username, then the viewer cannot view drafts.
        if username.is_none() {
            return Ok(false);
        }

        // Otherwise there is a username and we should send an API query.
        let query_vars: Variables = Variables {
            // Use the current local date.
            now: Local::today().naive_local(),
            // Unwrap the username
            username: username.unwrap()
        };

        // Call the API.
        let api_response: ResponseData = send_query::<Self>(query_vars).await?;

        // First check if the user is a faculty advisor.
        let user_role: UserRole = api_response.users_by_pk.map(|user| user.role).unwrap_or(UserRole::Student);
        if user_role == UserRole::FacultyAdvisor {
            return Ok(true);
        }

        // If they are not a faculty advisor, check if they are a current coordinator.
        let is_current_coordinator: bool = api_response
            // Start by flattening all the current semesters into one list of coordinator flags.
            .current_semesters
            .into_iter()
            .map(|semester| semester.enrollments)
            .flatten()
            .map(|enrollment| enrollment.is_coordinator)
            // And check if any of them are true
            .any(|is_coordinator| is_coordinator);

        return Ok(is_current_coordinator);
    }
}
