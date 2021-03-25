//! Meetings page template. This is mostly a static template.

use crate::templates::Template;
use crate::web::api::rcos::meetings::get::meetings::MeetingsMeetings;
use crate::web::services::meetings::MeetingsQuery;

/// The path to the template's handlebars file.
const TEMPLATE_NAME: &'static str = "meetings_page";

/// The handlebars key for the list of meetings from the RCOS API response.
pub const MEETINGS: &'static str = "meetings";

/// The handlebars key for the query parameters (optional).
pub const QUERY: &'static str = "query";

/// Make a meetings page template.
pub fn make(events: Vec<MeetingsMeetings>, query: Option<MeetingsQuery>) -> Template {
    Template::new(TEMPLATE_NAME)
        .field(MEETINGS, events)
        .field(QUERY, query)
}
