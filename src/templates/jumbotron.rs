use crate::{
    web::RequestContext,
    templates::Template
};

/// A template for a jumbotron.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Jumbotron {
    /// The large text (jumbotron heading)
    heading: String,
    /// The message explaining the heading.
    message: String,
}

impl Jumbotron {
    /// The template path from the templates directory.
    const TEMPLATE_NAME: &'static str = "jumbotron";

    /// Construct a jumbotron.
    fn new(heading: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            heading: heading.into(),
            message: message.into(),
        }
    }

    /// Get a page with a jumbotron in it.
    pub async fn jumbotron_page(
        ctx: &RequestContext,
        page_title: &str,
        heading: impl Into<String>,
        message: impl Into<String>,
    ) -> String {
        let jumbotron: Template = Jumbotron::new(heading, message).into();
        ctx.render_in_page(jumbotron, page_title).await
    }
}

impl Into<Template> for Jumbotron {
    fn into(self) -> Template {
        Template::new(Self::TEMPLATE_NAME)
            .with_fields(self)
    }
}