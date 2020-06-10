use handlebars::{Handlebars, RenderError};
use crate::templates::navbar::Navbar;


/// A page on the RCOS website.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Page {
    /// The page title.
    page_title: String,
    /// The navbar at the top of the page.
    navbar: Navbar,
    /// The inner html for this webpage. This is rendered unescaped. Do not let the user get stuff
    /// Ensure that no user input gets rendered into this unescaped (as it will create an XSS vulnerability).
    page_body: String,
}

impl Page {
    /// Create a new web page.
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            page_title: title.into(),
            page_body: body.into(),
            navbar: Navbar::with_defaults()
        }
    }

    /// Render the page to html using the given registry.
    pub fn render(&self, registry: &Handlebars) -> Result<String, RenderError> {
        registry.render("page", self)
    }
}