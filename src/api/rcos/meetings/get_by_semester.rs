/// Type representing public RCOS meetings.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/get.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct GetMeetingsBySemester;

use crate::api::rcos::{prelude::*, send_query};
use crate::error::TelescopeError;
use self::get_meetings_by_semester::{GetMeetingsBySemesterMeetings, Variables};

impl GetMeetingsBySemester {
    /// Get all the meetings for a given semester ID.
    pub async fn execute(semester_id: String) -> Result<Vec<GetMeetingsBySemesterMeetings>, TelescopeError> {
        Ok(send_query::<Self>(Variables { semester_id })
            .await?
            .meetings)
    }
}
