use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};
use handlebars::Handlebars;
use std::{
    sync::Arc,
    path::PathBuf
};
use lettre::{
    SmtpClient,
};
use crate::env::{
    CONFIG,
    ConcreteConfig
};

/// Struct to store shared app data and objects.
#[derive(Clone)]
pub struct AppData {
    /// The handlebars template registry.
    pub template_registry: Arc<Handlebars<'static>>,
    /// Database connection pool
    db_connection_pool: Pool<ConnectionManager<PgConnection>>,
    /// SMTP Mailer client config (if in use).
    smtp_client: Option<SmtpClient>,
    /// Path for file mailer if in use.
    file_mailer_path: Option<PathBuf>,
    /// Should mail stubs be created?
    use_stub_mailer: bool,
}

impl AppData {
    /// Create new App Data object using the global static config.
    pub fn new()-> Self {
        let config: &ConcreteConfig = &*CONFIG;
        // register handlebars templates
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
        info!("Handlebars templates registered.");

        // Set up database connection pool.
        let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
        let pool = diesel::r2d2::Pool::builder()
            // max 12 connections at once
            .max_size(12)
            // if a connection cannot be pulled from the pool in 20 seconds, timeout
            .connection_timeout(std::time::Duration::from_secs(20))
            .build(manager)
            .map_err(|e| {
                error!("Could not create database connection pool {}", e);
                e
            })
            .unwrap();
        info!("Created database connection pool.");

        Self {
            template_registry: Arc::new(template_registry),
            db_connection_pool: pool,
            use_stub_mailer: config.email_config.stub,
            file_mailer_path: config.email_config.file.clone(),
            smtp_client: config.email_config.smtp.clone().map(|c| {
                // FIXME: Create SMTP client here
                unimplemented!()
            })
        }
    }

    /// Get a clone of the database connection pool (which internally uses an Arc,
    /// so this is just a reference copy).
    pub fn clone_db_conn_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        self.db_connection_pool.clone()
    }
}
