//! GraphQL query to get a meeting by its ID.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing public RCOS meetings.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/get.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct GetMeetingById;

use self::get_meeting_by_id::{GetMeetingByIdMeeting, Variables};

impl GetMeetingById {
    /// Get a meeting by its ID.
    pub async fn get(meeting_id: i64) -> Result<Option<GetMeetingByIdMeeting>, TelescopeError> {
        Ok(send_query::<Self>(Variables { id: meeting_id })
            // Wait for API response
            .await?
            // Extract the meeting object.
            .meeting)
    }
}

impl GetMeetingByIdMeeting {
    /// Get the title of this meeting. This is the user-defined title if there is one, otherwise
    /// a title is constructed from the start date and meeting type.
    pub fn title(&self) -> String {
        // Check for a user-defined title.
        if self.title.is_some() {
            return self.title.clone().unwrap();
        }

        // Otherwise create a title.
        format!(
            "RCOS {} - {}",
            self.type_,
            self.start_date_time.format("%B %_d, %Y")
        )
    }
}
