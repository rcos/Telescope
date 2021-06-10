//! GraphQL query to get navbar authentication info on a user.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::api::rcos::users::UserRole;
use crate::error::TelescopeError;
use chrono::Utc;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/navbar_authentication.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Authentication;

impl Authentication {
    /// Get the navbar authentication object for a user.
    pub async fn get(username: String) -> Result<authentication::ResponseData, TelescopeError> {
        send_query::<Self>(authentication::Variables {
            username,
            now: Utc::today().naive_utc(),
        })
        .await
    }
}

impl authentication::ResponseData {
    /// Is this user coordinating currently?
    pub fn is_coordinating(&self) -> bool {
        self.users_by_pk
            .as_ref()
            .map(|u| u.is_current_coordinator.len() > 0)
            .unwrap_or(false)
    }

    /// Is this user currently mentoring?
    pub fn is_mentoring(&self) -> bool {
        self.users_by_pk
            .as_ref()
            .map(|u| u.is_current_mentor.len() > 0)
            .unwrap_or(false)
    }

    /// Is this user an admin (either faculty advisor or sysadmin)
    pub fn is_admin(&self) -> bool {
        self.users_by_pk
            .as_ref()
            .map(|u| u.role == UserRole::FacultyAdvisor || u.role == UserRole::Sysadmin)
            .unwrap_or(false)
    }

    /// Is this user's role student?
    pub fn is_student(&self) -> bool {
        self.users_by_pk
            .as_ref()
            .map(|u| u.role == UserRole::Student)
            .unwrap_or(false)
    }
}
