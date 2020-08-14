
/// A template for adding an alert to another page.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct WithAlert {
    alert_text: String,
    alert_class: String,
    content: String
}

impl WithAlert {
    /// Render a template and construct an alert above it.
    fn new(alert_text: String, alert_class: String, content: String) -> Self {
        Self {
            alert_text,
            alert_class,
            content
        }
    }
}