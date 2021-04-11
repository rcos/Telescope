//! Meeting page template.

use crate::templates::Template;
use crate::web::api::rcos::meetings::get_by_id::meeting::ResponseData;

/// The path from the templates directory to this template.
const TEMPLATE_NAME: &'static str = "meetings/page";

/// Handlebars key for the meeting data returned by the central RCOS API.
pub const MEETING: &'static str = "meeting";

// /// Make a meeting page template for a meeting.
// pub fn make(meeting)

