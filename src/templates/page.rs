use crate::error::TelescopeError;
use crate::templates::Template;
use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{Ready, ready};
use crate::templates::navbar::Navbar;
use crate::templates::tags::Tags;

/// The template for a page shown to the user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Page {
    /// The page title. Displayed on the tab in browser. Not necessarily the
    /// same as the OGP title.
    pub title: String,

    /// The value used to render the navbar.
    pub navbar: Navbar,

    /// The content that will be rendered in the page.
    pub content: Template,

    /// The current telescope version.
    version: String,

    /// Open Graph Protocol tags.
    pub ogp_tags: Tags,
}

impl Page {
    /// The path to the page template from the templates directory.
    const TEMPLATE_PATH: &'static str = "page";

    /// Create a page. Use default OGP tags. Call API to determine navbar privileges.
    pub async fn new(request: &HttpRequest, title: impl Into<String>, content: Template) -> Result<Self, TelescopeError> {
        Ok(Page {
            title: title.into(),
            navbar: Navbar::for_request(request).await?,
            content,
            version: env!("CARGO_PKG_VERSION").to_string(),
            ogp_tags: Tags::for_request(request)
        })
    }

    /// Render the page content and turn the page object into a template object.
    pub fn as_template(&self) -> Result<Template, TelescopeError> {
        // Render the page content.
        let content_rendered: String = self.content.render()?;
        // Turn this object into a JSON value.
        let mut template = Template::new(Self::TEMPLATE_PATH);
        // Set the fields of the template to this object.
        template.fields = json!(&self);
        // Replace the content field with the rendered content.
        template["content"] = json!(content_rendered);
        // Return the template.
        return Ok(template);
    }

    /// Render this page into a string using the handlebars template registry.
    pub fn render(&self) -> Result<String, TelescopeError> {
        self.as_template()?.render()
    }
}

// Implement responder for page so that we can return pages from services/handlers.
impl Responder for Page {
    type Error = TelescopeError;
    type Future = Ready<Result<HttpResponse, Self::Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        match self.as_template() {
            // If content can be rendered, respond using the normal template responder.
            Ok(template) => template.respond_to(req),
            // Otherwise return the error immediately.
            Err(err) => ready(Err(err))
        }
    }
}
