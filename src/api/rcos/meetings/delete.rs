//! Meeting deletion mutation.

use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing GraphQL mutation to delete a meeting and associated attendances.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/delete.graphql",
    response_derives = "Debug,Clone,Serialize",
    variables_derives = "Debug,Clone,Copy"
)]
pub struct DeleteMeeting;

impl DeleteMeeting {
    /// Delete a meeting and all associated attendances.
    pub async fn execute(meeting_id: i64) -> Result<delete_meeting::ResponseData, TelescopeError>{
        send_query::<Self>(delete_meeting::Variables { meeting_id }).await
    }
}
