use crate::web::{Template, RequestContext};
use crate::templates::page::Page;

/// A template for adding an alert to another page.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct WithAlert {
    alert_content: String,
    alert_class: String,
    content: String
}

impl WithAlert {
    /// Construct an alert.
    fn new(
        alert_content: impl Into<String>,
        alert_class: impl Into<String>,
        content: impl Into<String>
    ) -> Self {
        Self {
            alert_content: alert_content.into(),
            alert_class: alert_class.into(),
            content: content.into()
        }
    }

    /// Put an alert above a template and render to html.
    pub fn on_template<T: Template>(
        renderer: &RequestContext,
        alert_class: impl Into<String>,
        alert_content: impl Into<String>,
        base: &T
    ) -> String {
        let base_rendered = renderer.render(base);
        let w_alert = Self::new(alert_content, alert_class, base_rendered);
        renderer.render(&w_alert)
    }

    /// Render an alert above a template and then place the entire thing in a
    /// page template (adding the navbar above it and the footer below it).
    pub fn render_into_page<T: Template>(
        renderer: &RequestContext,
        page_title: impl Into<String>,
        alert_class: impl Into<String>,
        alert_content: impl Into<String>,
        base: &T
    ) -> String {
        let page_content = Self::on_template(renderer, alert_class, alert_content, base);
        let page_template = Page::new(page_title, page_content, renderer);
        renderer.render(&page_template)
    }
}

impl Template for WithAlert {
    const TEMPLATE_NAME: &'static str = "with_alert";
}