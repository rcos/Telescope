//! GraphQL mutation to create a meeting.

use crate::api::rcos::meetings::MeetingType;
use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;
use chrono::{DateTime, Utc};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/creation/create.graphql",
    response_derives = "Debug,Copy,Clone,Serialize"
)]
pub struct CreateMeeting;

/// Trim the whitespace off a string. If the trimmed string is empty default to None.
pub fn normalize_url(url: Option<String>) -> Option<String> {
    url.and_then(|string| (!string.trim().is_empty()).then(|| string))
}

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
        kind: MeetingType,
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
            // Coerce an empty or whitespace string to none.
            meeting_url: normalize_url(meeting_url),
            recording_url: normalize_url(recording_url),
            external_slides_url: normalize_url(external_slides_url),
            semester_id,
            kind,
        })
        .await
        .map(|response| response.insert_meetings_one.map(|obj| obj.meeting_id))
    }
}
