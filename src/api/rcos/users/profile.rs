//! Profile query.

use crate::api::rcos::{prelude::*, send_query};
use crate::error::TelescopeError;
use chrono::Utc;

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
    pub async fn for_user(
        target: String,
        viewer: Option<String>,
    ) -> Result<ResponseData, TelescopeError> {
        // Convert viewer to a vec with one or zero usernames in it.
        let viewer = viewer.map(|v| vec![v]).unwrap_or(Vec::new());

        // Send the query and await the response.
        send_query::<Self>(Variables {
            target,
            viewer,
            now: Utc::today().naive_utc(),
        })
        .await
    }
}
