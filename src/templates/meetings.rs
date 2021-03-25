//! Meetings page template. This is mostly a static template.

use crate::templates::Template;
use crate::web::api::rcos::meetings::get::meetings::MeetingsMeetings;

/// The path to the template's handlebars file.
const TEMPLATE_NAME: &'static str = "meetings_page";

/// The handlebars key for the list of meetings from the RCOS API response.
pub const MEETINGS: &'static str = "meetings";

/// Make a meetings page template.
pub fn make(events: Vec<MeetingsMeetings>) -> Template {
    Template::new(TEMPLATE_NAME)
        .field(MEETINGS, events)
}
