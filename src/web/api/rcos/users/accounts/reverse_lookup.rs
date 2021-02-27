//! RCOS API query to get the username (if available) of a user by platform and account id.

// Import and rename for GraphQL macro
use crate::web::api::rcos::users::UserAccountType as user_account;

/// Type representing query for username given a platform and user id on that
/// platform.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/users/accounts/reverse_lookup.graphql",
)]
pub struct ReverseLookup;

use reverse_lookup::ResponseData;

impl ResponseData {
    /// Get the username of a user (if they exist) via their
    /// account id for a given platform.
    pub fn username(mut self) -> Option<String> {
        Some(self.user_accounts.pop()?.username)
    }
}
