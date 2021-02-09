use crate::templates::Template;

/// The template path from the templates directory.
const TEMPLATE_NAME: &'static str = "jumbotron";

/// The handlebars key for the large text heading of the page.
pub const HEADING: &'static str = "heading";

/// The handlebars key for the smaller text message under the heading.
pub const MESSAGE: &'static str = "message";

/// Create a new jumbotron template.
pub fn new(heading: impl Into<String>, message: impl Into<String>) -> Template {
    Template::new(TEMPLATE_NAME)
        .field(HEADING, heading.into())
        .field(MESSAGE, message.into())
}
