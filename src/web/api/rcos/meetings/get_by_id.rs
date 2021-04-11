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
    ResponseData,
    MeetingMeeting,
    MeetingViewer,
    MeetingCurrentSemester
};

/// Converted response data from the RCOS API.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConvertedResponseData {
    /// The meeting object (if it exists).
    pub meeting: Option<MeetingMeeting>,
    /// The user viewing meeting page.
    pub viewer: Option<MeetingViewer>,
    /// The current semester as reported by the RCOS API.
    pub current_semester: Option<MeetingCurrentSemester>,
}

impl From<ResponseData> for ConvertedResponseData {
    fn from(rdata: ResponseData) -> Self {
        // Destructure response data.
        let ResponseData {
            mut current_semester,
            mut viewer,
            meeting,
        } = rdata;

        // Convert the fields from single-item lists to options.
        ConvertedResponseData {
            meeting,
            viewer: viewer.pop(),
            current_semester: current_semester.pop(),
        }
    }
}

impl Meeting {
    /// Get the meetings between two times, optionally filter to public meetings only.
    pub async fn get_by_id(meeting_id: i64, viewer_username: Option<String>) -> Result<ConvertedResponseData, TelescopeError> {
        Ok(send_query::<Self>(Variables { id: meeting_id, viewer: viewer_username })
            // Wait for API response
            .await?
            // Convert response data
            .into())
    }
}
