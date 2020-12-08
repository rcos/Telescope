use crate::{
    web::RequestContext,
    templates::{
        Template,
    }
};

/// The path to the jumbotron template from the template directory.
const TEMPLATE_PATH: &'static str = "jumbotron";

/// The large text heading at the top of the jumbotron.
pub const HEADING: &'static str = "heading";

/// The smaller text message under the heading.
pub const MESSAGE: &'static str = "message";

/// Construct a new jumbotron template.
pub fn new(heading: impl Into<String>, message: impl Into<String>) -> Template {
    Template::new(TEMPLATE_PATH)
        .field(HEADING, heading.into())
        .field(MESSAGE, message.into())
}
