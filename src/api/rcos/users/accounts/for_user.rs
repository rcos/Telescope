//! Lookup all the user accounts for a given user.

// Namespacing
use crate::api::rcos::prelude::*;
use crate::api::rcos::users::UserAccountType as user_account;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/accounts/for_user.graphql"
)]
pub struct UserAccounts;

use crate::api::rcos::send_query;
use crate::error::TelescopeError;
use user_accounts::Variables;

impl UserAccounts {
    /// Create the parameters for an accounts lookup query.
    fn make_variables(user_id: uuid) -> Variables {
        Variables { user_id }
    }

    /// Send a lookup query for a user's linked accounts.
    pub async fn send(user_id: uuid) -> Result<Vec<(user_account, String)>, TelescopeError> {
        send_query::<Self>(Self::make_variables(user_id))
            .await
            .map(|response| {
                response
                    .user_accounts
                    .into_iter()
                    .map(|linked_account| (linked_account.type_, linked_account.account_id))
                    .collect()
            })
    }
}
