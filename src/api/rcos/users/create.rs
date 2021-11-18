//! RCOS API mutation to create a user record and user_account record with it.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing GraphQL mutation to create a user and a user account.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/create_one.graphql"
)]
pub struct CreateOneUser;

impl CreateOneUser {
    /// Create a user and return the created user ID if this call did not fail.
    pub async fn execute(
        first_name: String,
        last_name: String,
        role: user_role,
        platform: user_account,
        platform_id: String,
    ) -> Result<Option<uuid>, TelescopeError> {
        send_query::<Self>(create_one_user::Variables {
            first_name,
            last_name,
            role,
            platform,
            platform_id,
        })
        .await
        .map(|response| response.insert_users_one.map(|obj| obj.id))
    }
}
