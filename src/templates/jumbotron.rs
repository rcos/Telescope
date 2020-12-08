use crate::{
    web::RequestContext,
    templates::{
        Template,
        page
    }
};
use serde_json::Value;

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

/// Construct a jumbotron in a page.
pub async fn page(
    ctx: &RequestContext,
    title: impl Into<Value>,
    heading: impl Into<String>,
    message: impl Into<String>
) -> Template {
    let jumbotron: Template = new(heading, message);
    page::of(ctx, title, &jumbotron).await
}

/// Construct and render a jumbotron in a page.
pub async fn rendered_page(
    ctx: &RequestContext,
    title: impl Into<Value>,
    heading: impl Into<String>,
    message: impl Into<String>
) -> String {
    ctx.render(&page(ctx, title, heading, message).await)
}