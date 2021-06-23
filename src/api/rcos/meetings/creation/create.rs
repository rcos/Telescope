//! GraphQL mutation to create a meeting.

use crate::api::rcos::prelude::*;
use crate::error::TelescopeError;
use crate::api::rcos::send_query;
use chrono::{DateTime, Utc};
use crate::api::rcos::meetings::MeetingType;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/creation/create.graphql",
    response_derives = "Debug,Copy,Clone,Serialize"
)]
pub struct CreateMeeting;

impl CreateMeeting {
    /// Execute a meeting creation mutation. Return the created meeting's ID.
    pub async fn execute(
        host_username: Option<String>,
        title: Option<String>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        description: String,
        is_draft: bool,
        is_remote: bool,
        location: Option<String>,
        meeting_url: Option<String>,
        recording_url: Option<String>,
        external_slides_url: Option<String>,
        semester_id: String,
        kind: MeetingType
    ) -> Result<Option<i64>, TelescopeError> {
        send_query::<Self>(create_meeting::Variables {
            host_username,
            title,
            start,
            end,
            description,
            is_draft,
            is_remote,
            location,
            meeting_url,
            recording_url,
            external_slides_url,
            semester_id,
            kind
        }).await.map(|response| response.insert_meetings_one.map(|obj| obj.meeting_id))
    }
}
