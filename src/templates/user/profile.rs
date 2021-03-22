//! User profile template functions and constants.

use crate::templates::Template;

/// The path from the template directory to the profile template.
const TEMPLATE_NAME: &'static str = "user/profile";

/// Handlebars key for the user's name.
pub const NAME: &'static str = "name";

/// Make a profile template for a user.
pub fn make(username: impl Into<String>) -> Template {
    Template::new(TEMPLATE_NAME)
        // Add the user's name
        .field(NAME, username.into())
}
