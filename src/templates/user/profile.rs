//! User profile template functions and constants.

use crate::templates::Template;
use crate::web::api::rcos::users::profile::profile::ProfileTarget;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// The path from the template directory to the profile template.
const TEMPLATE_NAME: &'static str = "user/profile";

/// The handlebars key for the user object returned from the database.
pub const TARGET: &'static str = "target";

/// Handlebars key for the viewers username (optional).
pub const VIEWER_USERNAME: &'static str = "viewer_username";

/// Make a profile template for a user. Query platform APIs as needed.
pub fn make(target: &ProfileTarget, viewer_username: Option<String>) -> Template {
    Template::new(TEMPLATE_NAME)
        .field(TARGET, target)
        .field(VIEWER_USERNAME, viewer_username)
}
