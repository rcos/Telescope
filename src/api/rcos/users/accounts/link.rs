//! Mutation to insert or update a user account record.

// Namespace items for generated code
use crate::api::rcos::users::{UserAccountType as user_account, UserAccountType};
use crate::api::rcos::{prelude::*, send_query};
use crate::error::TelescopeError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/accounts/link.graphql"
)]
pub struct LinkUserAccount;

use link_user_account::{ResponseData, Variables};

impl LinkUserAccount {
    /// Make the variables for a user account upsert mutation.
    fn make_variables(user_id: uuid, platform: UserAccountType, platform_id: String) -> Variables {
        Variables {
            user_id,
            platform,
            platform_id,
        }
    }

    /// Create a user account record on behalf of a user. This method will send a
    /// [`LinkUserAccount`] mutation with the subject set to the user ID. This method returns
    /// the user ID associated with the created user account (which should match the supplied
    /// user ID).
    ///
    /// This will fail if this user account is already linked to another user. In practice, this
    /// should be rare, so we let this case get handled by Telescope error propagation instead
    /// of accounting for it here.
    /// This will also fail if the account already exists. Please check to make sure the user
    /// has not already linked an account on this platform.
    pub async fn send(
        user_id: uuid,
        platform: UserAccountType,
        platform_id: String,
    ) -> Result<uuid, TelescopeError> {
        send_query::<Self>(Self::make_variables(user_id, platform, platform_id))
            .await
            .map(ResponseData::user_id)
    }
}

impl ResponseData {
    /// Extract the user ID from this response (if there was one).
    fn user_id(self) -> uuid {
        self.insert_user_accounts_one
            .expect("This should not be null -- this mutation should always return data.")
            .user_id
    }
}
