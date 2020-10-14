use crate::templates::page::Page;
use crate::web::{RequestContext, Template};

/// A template for a jumbotron.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Jumbotron {
    /// The large text (jumbotron heading)
    heading: String,
    /// The message explaining the heading.
    message: String,
}

impl Jumbotron {
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
        page_title: impl Into<String>,
        heading: impl Into<String>,
        message: impl Into<String>,
    ) -> String {
        ctx.render(&Page::new(
            page_title.into(),
            ctx.render(&Jumbotron::new(heading, message)),
            ctx,
        ).await)
    }
}

impl Template for Jumbotron {
    const TEMPLATE_NAME: &'static str = "jumbotron";
}
