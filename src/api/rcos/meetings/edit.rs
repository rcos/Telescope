//! Meeting edit mutation and host selection query.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing GraphQL meeting edit mutation.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/edit/edit.graphql",
    response_derives = "Debug,Copy,Clone,Serialize"
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

/// Type representing host selection query used while editing meetings.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/edit/host_selection.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct EditHostSelection;

impl EditHostSelection {
    /// Get the available hosts for this meeting.
    pub async fn get(meeting_id: i64) -> Result<edit_host_selection::ResponseData, TelescopeError> {
        send_query::<Self>(edit_host_selection::Variables { meeting_id }).await
    }
}
