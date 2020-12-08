use crate::{
    templates::{
        navbar::Navbar,
        Template
    },
    web::RequestContext
};
use serde_json::Value;

/// The path to the page template from the templates directory.
const TEMPLATE_PATH: &'static str = "page";

/// The handlebars field to store the title.
pub const TITLE: &'static str = "title";

/// The handlebars field to store the navbar object.
pub const NAVBAR: &'static str = "navbar";

/// The handlebars field to store the content of the page.
pub const CONTENT: &'static str = "content";

/// The handlebars field to store the version that telescope
/// is currently running.
pub const VERSION: &'static str = "version";

/// Create a new template object to hold the page.
/// The content of the page is rendered here and must be re-rendered if updated.
pub async fn of(ctx: &RequestContext, title: impl Into<Value>, content: &Template) -> Template {
    Template::new(TEMPLATE_PATH)
        .field(TITLE, title.into())
        .field(NAVBAR, Navbar::from_context(ctx).await.template())
        .field(CONTENT, ctx.render(content))
        .field(VERSION, env!("CARGO_PKG_VERSION"))
}
