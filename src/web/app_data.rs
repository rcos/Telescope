use handlebars::Handlebars;
use std::sync::Arc;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;

/// Struct to store shared app data and objects.
#[derive(Clone)]
pub struct AppData {
    /// The handlebars template registry.
    pub template_registry: Arc<Handlebars<'static>>,
    /// Database thread pool
    pub db_connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl AppData {
    /// Create new App Data object.
    pub fn new(templates: Handlebars<'static>, pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            template_registry: Arc::new(templates),
            db_connection_pool: pool
        }
    }
}
