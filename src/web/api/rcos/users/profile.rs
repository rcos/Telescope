//! Profile query.

use chrono::{DateTime, Utc};
use crate::error::TelescopeError;
use crate::web::api::rcos::send_query;

// Namespaced types for generated code
use crate::web::api::rcos::users::{
    // UserAccountType as user_account,
    UserRole as user_role
};

// Ignore the compiler warning this style would generate.
#[allow(nonstandard_style)]
type timestamptz = DateTime<Utc>;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/profile.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Profile;

// import generated types.
use profile::{
    Variables,
    ResponseData,
    //ProfileUsersByPk
};

impl Profile {
    /// Get the profile data for a given username.
    pub async fn for_user(username: String) -> Result<ResponseData, TelescopeError> {
        send_query::<Self>(Variables { username }).await
    }
}
