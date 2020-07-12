use handlebars::Handlebars;
use std::sync::Arc;

/// Struct to store shared app data and objects.
#[derive(Clone)]
pub struct AppData {
    /// The handlebars template registry.
    pub template_registry: Arc<Handlebars<'static>>,
}

impl AppData {
    /// Create new App Data object.
    pub fn new(templates: Handlebars<'static>) -> Self {
        Self {
            template_registry: Arc::new(templates),
        }
    }
}
