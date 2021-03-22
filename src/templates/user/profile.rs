//! User profile template functions and constants.

use crate::templates::Template;

/// The path from the template directory to the profile template.
const TEMPLATE_NAME: &'static str = "user/profile";

/// Handlebars key for the user's name.
pub const NAME: &'static str = "name";

pub fn make() -> Template {
    unimplemented!()
}
