use crate::templates::Template;
use crate::templates::navbar;
use serde_json::Value;
use crate::app_data::AppData;
use crate::error::TelescopeError;
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
pub fn of(
    req: &HttpRequest,
    title: impl Into<Value>,
    content: &Template
) -> Result<Template, TelescopeError> {
    let content_rendered = AppData::global().render_template(content)?;
    Ok(Template::new(TEMPLATE_PATH)
        .field(TITLE, title.into())
        .field(NAVBAR, navbar::userless(req))
        .field(CONTENT, content_rendered)
        .field(VERSION, env!("CARGO_PKG_VERSION")))
}
