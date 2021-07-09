//! Meeting edit mutation.

use crate::api::rcos::prelude::*;
use crate::error::TelescopeError;
use crate::api::rcos::send_query;

/// Type representing GraphQL meeting edit mutation.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/edit/edit.graphql",
    response_derives = "Debug,Copy,Clone,Serialize",
)]
pub struct EditMeeting;

impl EditMeeting {
    /// Execute a meeting edit mutation. Return the ID of the edited meeting if any
    /// changes were made.
    pub async fn execute(vars: edit_meeting::Variables) -> Result<Option<i64>, TelescopeError> {
        send_query::<Self>(vars)
            .await
            .map(|response| response.update_meetings_by_pk.map(|obj| obj.meeting_id))
    }
}
