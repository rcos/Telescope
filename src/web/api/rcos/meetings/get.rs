//! Public meetings query.

use crate::web::api::rcos::prelude::*;
use chrono::{DateTime, TimeZone, Utc, Duration};
use crate::error::TelescopeError;
use crate::web::api::rcos::send_query;
use crate::web::services::calendar::FullCalendarEvent;

/// Type representing public RCOS meetings.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/get.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Meetings;

use self::meetings::{
    Variables,
    ResponseData,
    MeetingsMeetings
};

impl Meetings {
    /// Get the meetings between two times, optionally filter to public meetings only.
    pub async fn get(start: DateTime<Utc>, end: DateTime<Utc>, public_only: bool) -> Result<Vec<FullCalendarEvent>, TelescopeError> {
        Ok(send_query::<Self>(Variables { start, end, public_only })
            .await?
            .meetings
            .into_iter()
            .map(|meeting: MeetingsMeetings| meeting.into())
            .collect())
    }
}

impl Into<FullCalendarEvent> for MeetingsMeetings {
    fn into(self) -> FullCalendarEvent {
        // Extract fields.
        let start: DateTime<Utc> = self.start_date_time;
        let end: DateTime<Utc> = self.end_date_time;
        let id: i64 = self.meeting_id;
        let variant: meeting_type = self.type_;
        let meeting_url: Option<&String> = self.meeting_url.as_ref();
        let recording_url: Option<&String> = self.recording_url.as_ref();

        // Resolve the title
        let title: Option<String> = self.title.clone();
        let title: String = title
            // Format default as "variant date" (i.e. "Large Group March 23, 2021")
            .unwrap_or(format!("{} {}", variant, start
                .naive_local()
                .date()
                // Format as month day, year (i.e May 1, 2021)
                .format("%B %_d, %Y")));

        // TODO: eventually support generated meeting slides on telescope.
        // Resolve the url
        let url: Option<String> = meeting_url
            // Use the recording URL as a secondary
            .or(recording_url)
            // Convert to string.
            .map(String::from);

        return FullCalendarEvent { id, title, start, end, url, source: self};
    }
}