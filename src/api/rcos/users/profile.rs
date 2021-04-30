//! Profile query.

use crate::error::TelescopeError;
use crate::api::rcos::{prelude::*, send_query};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/profile.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Profile;

// import generated types.
use profile::{ResponseData, Variables};

impl Profile {
    /// Get the profile data for a given username.
    pub async fn for_user(username: String) -> Result<ResponseData, TelescopeError> {
        send_query::<Self>(Variables { username }).await
    }
}
