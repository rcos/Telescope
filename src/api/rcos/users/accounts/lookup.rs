//! Lookup an account by the type and user ID.

use crate::api::rcos::{prelude::*, send_query};
use crate::error::TelescopeError;

/// GraphQL query to lookup a user account by type and user ID.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/accounts/lookup.graphql"
)]
pub struct AccountLookup;

use self::account_lookup::{ResponseData, Variables};

impl AccountLookup {
    /// Make the variables for an account lookup query.
    pub fn make_variables(user_id: uuid, platform: user_account) -> Variables {
        Variables { user_id, platform }
    }

    /// Send the account lookup query. This return the user's ID on the given platform if there
    /// is one linked.
    pub async fn send(
        user_id: uuid,
        platform: user_account,
    ) -> Result<Option<String>, TelescopeError> {
        // Send the query and convert the response.
        send_query::<Self>(Self::make_variables(user_id, platform))
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
