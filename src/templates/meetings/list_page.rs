//! Template for page listing of meetings.

use crate::templates::Template;
use crate::web::api::rcos::meetings::get::meetings::MeetingsMeetings;
use crate::web::services::meetings::MeetingsQuery;
use crate::web::api::rcos::meetings::authorization_for::UserMeetingAuthorization;

/// The path to the template's handlebars file.
const TEMPLATE_NAME: &'static str = "meetings/list";

/// The handlebars key for the list of meetings from the RCOS API response.
pub const MEETINGS: &'static str = "meetings";

/// The handlebars key for the query parameters (optional).
pub const QUERY: &'static str = "query";

/// The handlebars key for the viewer's authorization info.
pub const AUTHORIZATION: &'static str = "authorization";

/// Make a meetings page template.
pub fn make(
    events: Vec<MeetingsMeetings>,
    query: Option<MeetingsQuery>,
    authorization: &UserMeetingAuthorization
) -> Template {
    Template::new(TEMPLATE_NAME)
        .field(MEETINGS, events)
        .field(QUERY, query)
        .field(AUTHORIZATION, authorization)
}
