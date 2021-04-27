//! Module bridging to template used to render meeting title. This template is mostly used as
//! a partial other templates, but can be called manually via this module.

use crate::templates::Template;
use crate::web::api::rcos::meetings::MeetingType;
use chrono::{DateTime, Utc};

/// Path to the handlebars template file from the templates directory.
const TEMPLATE_NAME: &'static str = "meetings/title";

/// Handlebars key for the user-defined title if there is one.
pub const TITLE: &'static str = "title";

/// Handlebars key for the meeting type.
pub const TYPE: &'static str = "type";

/// Handlebars key for the timestamp (with timezone) of the meeting's start.
pub const TIMESTAMP: &'static str = "start_date_time";

/// Make the template object that can be rendered into a meeting title.
pub fn of(
    meeting_title: &Option<String>,
    meeting_type: MeetingType,
    timestamp: DateTime<Utc>,
) -> Template {
    Template::new(TEMPLATE_NAME)
        .field(TITLE, meeting_title)
        .field(TYPE, meeting_type)
        .field(TIMESTAMP, timestamp)
}
