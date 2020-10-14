use crate::templates::navbar::Navbar;
use crate::web::context::Template;
use crate::web::RequestContext;

/// A page on the RCOS website.
#[derive(Clone, Debug, Serialize)]
pub struct Page {
    /// The page title.
    page_title: String,
    /// The navbar at the top of the page.
    navbar: Navbar,
    /// The inner html for this webpage. This is rendered unescaped. Do not let the user get stuff
    /// Ensure that no user input gets rendered into this unescaped (as it will create an XSS vulnerability).
    page_body: String,
    /// The version of this project.
    version: &'static str,
}

impl Page {
    /// Create a new web page.
    pub fn new(title: impl Into<String>, body: impl Into<String>, ctx: &RequestContext) -> Self {
        Self {
            page_title: title.into(),
            page_body: body.into(),
            navbar: Navbar::from_context(ctx),
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    /// Creates a page with a template rendered as the body.
    pub fn of<T: Template>(title: impl Into<String>, template: &T, ctx: &RequestContext) -> Self {
        Self::new(title, ctx.render(template), ctx)
    }
}

impl Template for Page {
    const TEMPLATE_NAME: &'static str = "page";
}
