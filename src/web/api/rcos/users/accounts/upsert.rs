//! Mutation to insert or update a user account record.

// Namespace items for generated code
use crate::web::api::rcos::users::{UserAccountType as user_account, UserAccountType};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/accounts/upsert.graphql"
)]
pub struct UpsertUserAccount;

use upsert_user_account::{
    Variables,
    ResponseData
};
use crate::error::TelescopeError;
use crate::web::api::rcos::send_query;

impl UpsertUserAccount {
    /// Make the variables for a user account upsert mutation.
    fn make_variables(username: String, platform: UserAccountType, platform_id: String) -> Variables {
        Variables { username, platform, platform_id }
    }


    /// Create or update a user account record on behalf of a user. This method will send an
    /// [`UpsertUserAccount`] mutation with the subject set to the username. This method returns
    /// the username associated with the created user account (whitch should match the supplied username).
    pub async fn send(username: String, platform: UserAccountType, platform_id: String) -> Result<String, TelescopeError> {
        send_query::<Self>(Some(username.clone()), Self::make_variables(username, platform, platform_id))
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