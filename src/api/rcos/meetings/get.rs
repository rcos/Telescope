//! List meetings query.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing public RCOS meetings.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/get.graphql",
    response_derives = "Debug,Clone,Serialize",
    variables_derives = "Default"
)]
pub struct Meetings;

use self::meetings::{MeetingsMeetings, Variables};

impl Meetings {
    /// Get the meetings satisfying a given filter.
    pub async fn get(variables: Variables) -> Result<Vec<MeetingsMeetings>, TelescopeError> {
        send_query::<Self>(variables)
            .await
            .map(|respponse| respponse.meetings)
    }
}
