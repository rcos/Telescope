use crate::env::{ConcreteConfig, CONFIG};
use crate::error::TelescopeError;
use crate::templates::helpers::register_helpers;
use actix_web::web::block;
use handlebars::Handlebars;
use std::{path::PathBuf, sync::Arc};

lazy_static! {
    /// Lazy Static to store app data at runtime.
    static ref APP_DATA: Arc<AppData> = {
        Arc::new(AppData::new())
    };
}

/// Struct to store shared app data and objects.
#[derive(Clone)]
pub struct AppData {
    /// The handlebars template registry.
    template_registry: Arc<Handlebars<'static>>,
}

impl AppData {
    /// Create new App Data object using the global static config.
    fn new() -> Self {
        let config: &ConcreteConfig = &*CONFIG;

        // Register handlebars templates
        let mut template_registry = Handlebars::new();
        template_registry
            .register_templates_directory(".hbs", "templates")
            .map_err(|e| {
                error!("Failed to properly register handlebars templates: {}", e);
                e
            })
            .unwrap();
        // Use handlebars strict mode so that we get an error when we try to render a
        // non-existent field
        template_registry.set_strict_mode(true);
        // Register the helpers defined in the helpers module.
        register_helpers(&mut template_registry);
        info!("Handlebars templates registered.");

        Self {
            template_registry: Arc::new(template_registry),
        }
    }

    /// Get an [`Arc`] reference to the global, lazily generated app-data.
    pub fn global() -> Arc<AppData> {
        APP_DATA.clone()
    }

    /// Get an [`Arc`] reference to the template registry.
    pub fn get_handlebars_registry(&self) -> Arc<Handlebars<'static>> {
        self.template_registry.clone()
    }
}
