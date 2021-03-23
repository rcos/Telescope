//! User profile template functions and constants.

use crate::templates::Template;
use crate::web::api::rcos::users::profile::profile::{
    ProfileTarget,
    ProfileTargetCoordinating,
    ProfileTargetMentoring
};
use std::collections::HashMap;
use crate::web::api::rcos::users::UserAccountType;
use crate::web::services::auth::identity::AuthenticationCookie;
use crate::error::TelescopeError;
use serde::Serialize;
use chrono::{DateTime, Utc};

/// The path from the template directory to the profile template.
const TEMPLATE_NAME: &'static str = "user/profile";

/// The handlebars key for the name of the user who owns the profile.
pub const NAME: &'static str = "name";

/// The handlebars key for the account creation string.
pub const CREATED_AT: &'static str = "created_at";

/// The handlebars key for the list of semesters this user has mentored.
pub const MENTORING: &'static str = "mentoring";

/// The handlebars key for the list of semesters this user was a coordinator.
pub const COORDINATING: &'static str = "coordinating";

/// Make a profile template for a user. Query platform APIs as needed.
pub fn make(
    name: impl Serialize,
    created_at: DateTime<Utc>,
    mentoring: &[ProfileTargetMentoring],
    coordinating: &[ProfileTargetCoordinating],
) -> Template {
    Template::new(TEMPLATE_NAME)
        .field(NAME, name)
        .field(MENTORING, mentoring)
        .field(COORDINATING, coordinating)
        // Convert the created at time to local time and
        // format into a string
        .field(CREATED_AT, created_at
            .naive_local()
            // Get just the date
            .date()
            // Format Month Day Year (e.g. July 1, 2020)
            .format("%B %_d, %Y")
            // Convert to string.
            .to_string())

}
