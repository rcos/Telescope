use crate::templates::Template;

/// Path from templates directory to the login template.
const TEMPLATE_PATH: &'static str = "login";

/// Create a new login page template.
pub fn new() -> Template {
    Template::new(TEMPLATE_PATH)
}
