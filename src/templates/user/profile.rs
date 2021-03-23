//! User profile template functions and constants.

use crate::templates::Template;
use crate::web::api::rcos::users::profile::profile::ProfileUsersByPk;

/// The path from the template directory to the profile template.
const TEMPLATE_NAME: &'static str = "user/profile";

/// Handlebars key for the RCOS API response data about the target user.
pub const TARGET: &'static str = "target";

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct TargetUser {
    /// The user's name.
    pub name: String,
    // /// Cohort of the user.
    // pub cohort: Option<i64>,
    /// String representing when the account was created.
    pub created_at: String,
}

/// Make a profile template for a user.
pub fn make(data: &TargetUser) -> Template {
    Template::new(TEMPLATE_NAME)
        // Add the user's name
        .field(TARGET, data)
}
