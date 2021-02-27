//! Lookup an account by the type and username.

// Import and rename for GraphQL macro
use crate::web::api::rcos::users::UserAccountType as user_account;

/// GraphQL query to lookup a user account by type and username.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/users/accounts/lookup.graphql",
)]
pub struct AccountLookup;

use self::account_lookup::ResponseData;

impl ResponseData {
    /// The id associated with a given RCOS user for a given platform (as specified
    /// in the query).
    pub fn platform_id(self) -> Option<String> {
        Some(self.user_accounts_by_pk?.account_id)
    }
}
