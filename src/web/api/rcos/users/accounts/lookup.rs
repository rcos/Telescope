//! Lookup an account by the type and username.

// Import and rename for GraphQL macro
use crate::web::api::rcos::users::UserAccountType as user_account;

/// GraphQL query to lookup a user account by type and username.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/accounts/lookup.graphql"
)]
pub struct AccountLookup;

use self::account_lookup::{
    Variables,
    ResponseData
};
use crate::error::TelescopeError;
use crate::web::api::rcos::send_query;

impl AccountLookup {
    /// Make the variables for an account lookup query.
    pub fn make_variables(username: String, platform: user_account) -> Variables {
        Variables { username, platform }
    }

    /// Send the account lookup query. This return the user's ID on the given platform if there
    /// is one linked.
    pub async fn send(username: String, platform: user_account) -> Result<Option<String>, TelescopeError> {
        // Send the query and convert the response.
        send_query::<Self>(Self::make_variables(username, platform))
            .await
            .map(|response| response.platform_id())
    }
}


impl ResponseData {
    /// The id associated with a given RCOS user for a given platform (as specified
    /// in the query).
    fn platform_id(self) -> Option<String> {
        Some(self.user_accounts_by_pk?.account_id)
    }
}
