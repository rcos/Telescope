//! GraphQL query to get the username of the host of a meeting by the meeting's ID.

use crate::api::rcos::send_query;
use crate::error::TelescopeError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/get_host.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct MeetingHost;

impl MeetingHost {
    /// Get the username of the host of a meeting if there is one.
    pub async fn get(meeting_id: i64) -> Result<Option<String>, TelescopeError> {
        send_query::<Self>(meeting_host::Variables { meeting_id })
            .await
            .map(|response| {
                response
                    .meetings_by_pk
                    .and_then(|meeting| meeting.host)
                    .map(|host| host.username)
            })
    }
}
