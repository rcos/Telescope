//! List meetings query.

use crate::error::TelescopeError;
use crate::api::rcos::meetings::MeetingType;
use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use chrono::{DateTime, Utc};

/// Type representing public RCOS meetings.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/get.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Meetings;

use self::meetings::{MeetingsMeetings, Variables};

impl Meetings {
    /// Get the meetings between two times, optionally filter to finalized meetings only.
    pub async fn get(
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        include_drafts: bool,
        accept_types: Vec<MeetingType>,
    ) -> Result<Vec<MeetingsMeetings>, TelescopeError> {
        Ok(send_query::<Self>(Variables {
            start,
            end,
            include_drafts,
            accept_types,
        })
        .await?
        .meetings)
    }
}
