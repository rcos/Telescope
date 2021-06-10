//! GraphQL lookup to get a user's role.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::api::rcos::users::UserRole;
use crate::error::TelescopeError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/role_lookup.graphql",
    response_derives = "Debug,Clone,Serialize,Copy"
)]
pub struct RoleLookup;

impl RoleLookup {
    /// Get a user's role. Return `Ok(None)` if there is no user record for this username.
    pub async fn get(username: String) -> Result<Option<UserRole>, TelescopeError> {
        send_query::<Self>(role_lookup::Variables { username })
            .await
            // Extract the role from the results
            .map(|result| result.users_by_pk.map(|u| u.role))
    }
}
