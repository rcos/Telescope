//! Mutation to unlink a user account.

// Namespace items for generated module
use crate::api::rcos::users::UserAccountType as user_account;
use crate::api::rcos::prelude::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/accounts/unlink.graphql"
)]
pub struct UnlinkUserAccount;

use crate::api::rcos::send_query;
use crate::error::TelescopeError;
use unlink_user_account::{ResponseData, Variables};

impl UnlinkUserAccount {
    /// Make variables for an unlink user-account mutation.
    fn make_variables(user_id: uuid, platform: user_account) -> Variables {
        Variables { user_id, platform }
    }

    /// Unlink and delete a user account from the database. Return the platform id
    /// of the account if it existed.
    ///
    /// This should be used with significant care, as a user record in the database with no linked
    /// accounts is orphaned and the user will not be able to login and use Telescope.
    pub async fn send(
        user_id: uuid,
        platform: user_account,
    ) -> Result<Option<String>, TelescopeError> {
        // Send the query, wait for and convert the response
        send_query::<Self>(Self::make_variables(user_id, platform))
            .await
            .map(ResponseData::platform_id)
    }
}

impl ResponseData {
    /// Get the on-platform ID of the account deleted.
    fn platform_id(self) -> Option<String> {
        Some(self.delete_user_accounts_by_pk?.account_id)
    }
}
