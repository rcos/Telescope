// ! GraphQL query to get the username of the host of a meeting by the meeting's ID.


use crate::error::TelescopeError;
use crate::api::rcos::send_query;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/get_members_by_id.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Members;

use self::members::{MembersSmallGroupMembers, Variables};

impl Members {
    /// Get the username of the host of a meeting if there is one.
    pub async fn get(
        id: i64,
    ) -> Result<Vec<MembersSmallGroupMembers>, TelescopeError> {
        Ok(send_query::<Self>(Variables {
            id,
        })
        .await?
        .small_group_members
        )
    }
}

