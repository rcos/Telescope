//! Meeting page template.

use crate::templates::Template;
use crate::web::api::rcos::meetings::get_by_id::meeting::{
    MeetingMeeting,
    MeetingViewer,
    MeetingCurrentSemester,
};

/// The path from the templates directory to this template.
const TEMPLATE_NAME: &'static str = "meetings/page";

/// Handlebars key for the meeting data returned by the RCOS API.
pub const MEETING: &'static str = "meeting";

/// Handlebars key for the viewer data returned by the RCOS API.
pub const VIEWER: &'static str = "viewer";

/// Handlebars key for the current semester data from the RCOS API.
pub const CURRENT_SEMESTER: &'static str = "current_semester";

/// Make a meeting page template for a meeting.
pub fn make(meeting: &MeetingMeeting, viewer: &Option<MeetingViewer>, current_semester: &Option<MeetingCurrentSemester>) -> Template {
    Template::new(TEMPLATE_NAME)
        .field(MEETING, meeting)
        .field(VIEWER, viewer)
        .field(CURRENT_SEMESTER, current_semester)
}
