//! Public meetings query.

use crate::web::{
    api::rcos::prelude::*,
    services::calendar::EventsQuery
};

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
    ResponseData
};
use chrono::{DateTime, TimeZone, Utc};
use crate::error::TelescopeError;

impl Meetings {
    /// Get the meetings between two times, optionally filter to public meetings only.
    pub async fn get(start: DateTime<Utc>, end: DateTime<Utc>, public_only: bool) -> Result<ResponseData, TelescopeError> {
        unimplemented!()
    }
}
