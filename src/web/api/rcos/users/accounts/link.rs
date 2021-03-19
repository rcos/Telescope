//! Mutation to insert or update a user account record.

// Namespace items for generated code
use crate::web::api::rcos::users::{UserAccountType as user_account, UserAccountType};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/accounts/link.graphql"
)]
pub struct LinkUserAccount;

use link_user_account::{
    Variables,
    ResponseData
};
use crate::error::TelescopeError;
use crate::web::api::rcos::send_query;

impl LinkUserAccount {
    /// Make the variables for a user account upsert mutation.
    fn make_variables(username: String, platform: UserAccountType, platform_id: String) -> Variables {
        Variables { username, platform, platform_id }
    }


    /// Create a user account record on behalf of a user. This method will send a
    /// [`LinkUserAccount`] mutation with the subject set to the username. This method returns
    /// the username associated with the created user account (which should match the supplied
    /// username).
    ///
    /// This will fail if this user account is already linked to another user. In practice, this
    /// should be rare, so we let this case get handled by Telescope error propagation instead
    /// of accounting for it here.
    /// This will also fail if the account already exists. Please check to make sure the user
    /// has not already linked an account on this platform.
    pub async fn send(username: String, platform: UserAccountType, platform_id: String) -> Result<String, TelescopeError> {
        send_query::<Self>(Self::make_variables(username, platform, platform_id))
            .await
            .map(ResponseData::username)
    }
}

impl ResponseData {
    /// Extract the username from this response (if there was one).
    fn username(self) -> String {
        self.insert_user_accounts_one
            .expect("This should not be null -- this mutation should always return data.")
            .username
    }
}