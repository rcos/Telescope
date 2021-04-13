//! List meetings query.

use crate::web::api::rcos::prelude::*;
use chrono::{DateTime, Utc};
use crate::error::TelescopeError;
use crate::web::api::rcos::send_query;

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
    MeetingsMeetings
};

impl Meetings {
    /// Get the meetings between two times, optionally filter to finalized meetings only.
    pub async fn get(start: DateTime<Utc>, end: DateTime<Utc>, include_drafts: bool) -> Result<Vec<MeetingsMeetings>, TelescopeError> {
        Ok(send_query::<Self>(Variables { start, end, include_drafts })
            .await?
            .meetings)
    }
}
