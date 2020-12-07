use crate::{
    templates::{
        navbar::Navbar,
        Template
    },
    web::RequestContext
};

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
    /// The template path from the templates directory.
    const TEMPLATE_NAME: &'static str = "page";

    /// Create a new web page.
    pub async fn new(
        title: impl Into<String>,
        body: impl Into<String>,
        ctx: &RequestContext,
    ) -> Self {
        Self {
            page_title: title.into(),
            page_body: body.into(),
            navbar: Navbar::from_context(ctx).await,
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    /// Creates a page with a template rendered as the body.
    pub async fn of(
        title: impl Into<String>,
        template: &Template,
        ctx: &RequestContext,
    ) -> Self {
        Self::new(title, ctx.render(template), ctx).await
    }
}


impl Into<Template> for Page {
    fn into(self) -> Template {
        Template::new(Self::TEMPLATE_NAME)
            .with_fields(self)
    }
}