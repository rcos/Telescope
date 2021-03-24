//! Public meetings query.

use crate::web::{
    api::rcos::prelude::*,
    services::calendar::EventsQuery
};

/// Type representing public RCOS meetings.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/public.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct PublicMeetings;

use self::public_meetings::{
    Variables,
    ResponseData
};

impl Into<Variables> for EventsQuery {
    fn into(self) -> Variables {
        Variables { start: self.start, end: self.end }
    }
}

