//! GraphQL query to get a meeting by its ID.

use crate::web::api::rcos::prelude::*;
use crate::web::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing public RCOS meetings.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/get_by_id.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Meeting;

use self::meeting::{
    Variables,
    MeetingMeetingsByPk
};

impl Meeting {
    /// Get the meetings between two times, optionally filter to public meetings only.
    pub async fn get_by_id(id: i64) -> Result<Option<MeetingMeetingsByPk>, TelescopeError> {
        Ok(send_query::<Self>(Variables { id }).await?.meetings_by_pk)
    }
}
