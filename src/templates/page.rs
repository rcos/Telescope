use crate::error::TelescopeError;
use crate::templates::navbar;
use crate::templates::Template;
use serde_json::Value;
use actix_web::HttpRequest;

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
pub async fn of(
    req: &HttpRequest,
    title: impl Into<Value>,
    content: &Template,
) -> Result<Template, TelescopeError> {
    // Render the content of this page
    let content_rendered: String = content.render()?;
    // Create the page.
    return with_content(req, title, content_rendered.as_str()).await;
}

/// Create a template with a content string rather than another template.
pub async fn with_content(
    req: &HttpRequest,
    title: impl Into<Value>,
    content: &str,
) -> Result<Template, TelescopeError> {
    // Build the rest of the page
    Ok(Template::new(TEMPLATE_PATH)
        .field(TITLE, title.into())
        .field(NAVBAR, navbar::for_request(req).await?)
        .field(CONTENT, content)
        .field(VERSION, env!("CARGO_PKG_VERSION")))
}
