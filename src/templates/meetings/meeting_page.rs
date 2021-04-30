//! Meeting page template.

use crate::api::rcos::meetings::authorization_for::UserMeetingAuthorization;
use crate::api::rcos::meetings::get_by_id::meeting::MeetingMeeting;
use crate::templates::Template;

/// The path from the templates directory to this template.
const TEMPLATE_NAME: &'static str = "meetings/page";

/// Handlebars key for the meeting data returned by the RCOS API.
pub const MEETING: &'static str = "meeting";

/// Handlebars key for the viewer's authorization info.
pub const AUTHORIZATION: &'static str = "auth";

/// Make a meeting page template for a meeting.
pub fn make(meeting: &MeetingMeeting, authorization: &UserMeetingAuthorization) -> Template {
    Template::new(TEMPLATE_NAME)
        .field(MEETING, meeting)
        .field(AUTHORIZATION, authorization)
}
