use crate::{
    templates::{
        navbar::Navbar,
        Template
    },
    web::RequestContext
};

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
pub async fn new(ctx: &RequestContext, title: &str, content: Template) -> Template {
    Template::new(TEMPLATE_PATH)
        .field(TITLE, title)
        .field(NAVBAR, Navbar::from_context(ctx).await.template())
        .field(CONTENT, content)
        .field(VERSION, env!("CARGO_PKG_VERSION"))
}
