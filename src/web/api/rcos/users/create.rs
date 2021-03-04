//! RCOS API mutation to create a user record and user_account record with it.

// Import and rename for GraphQL macro
use crate::web::api::rcos::users::{
    UserAccountType as user_account,
    UserRole as user_role
};

/// Type representing GraphQL mutation to create a user and a user account.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/users/create_one.graphql"
)]
pub struct CreateOneUser;

use create_one_user::{
    ResponseData,
    Variables
};

impl CreateOneUser {
    /// Make the input variables object for a user creation mutation.
    pub fn make_variables(
        username: String,
        first_name: String,
        last_name: String,
        role: user_role,
        platform: user_account,
        platform_id: String) -> Variables {
        Variables { username, first_name, last_name, role, platform, platform_id }
    }
}

impl ResponseData {
    /// Get the username that was added to the database.
    pub fn username(&self) -> Option<String> {
        Some(self.insert_users_one.as_ref()?.username.clone())
    }
}
