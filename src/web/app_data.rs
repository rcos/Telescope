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
    SmtpClient, SendableEmail, Transport, FileTransport, Envelope, EmailAddress,
    stub::StubTransport
};
use crate::env::{
    CONFIG,
    ConcreteConfig
};
use actix_web::web::block;
use uuid::Uuid;

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
    /// The email sender address.
    pub mail_sender_address: Option<Arc<EmailAddress>>,
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
            smtp_client: config.email_config.make_smtp_client(),
            mail_sender_address: config.email_config.address.as_ref().map(|s| Arc::new(s.clone()))
        }
    }

    /// Get a clone of the database connection pool (which internally uses an Arc,
    /// so this is just a reference copy).
    pub fn clone_db_conn_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        self.db_connection_pool.clone()
    }

    /// Send an email over the available mail transporters.
    pub async fn send_mail(&self, to: Vec<EmailAddress>, body: impl Into<String>) -> Result<(), ()> {
        let body = body.into();
        let envelope =
            Envelope::new(
                self.mail_sender_address.as_ref().map(|a| a.as_ref().clone()),
                to
            ).map_err(|e| {
                error!("Envelope Error: {}", e);
                ()
            })?;

        // randomly generate a new email id uuid
        let email_id = Uuid::new_v4()
            .to_hyphenated()
            .to_string()
            .to_lowercase();

        if self.use_stub_mailer {
            let email = SendableEmail::new(
                envelope.clone(),
                email_id.clone(),
                body.clone().into_bytes()
            );
            let mut transport = StubTransport::new_positive();
            transport.send(email).unwrap();
        }

        if let Some(dir) = self.file_mailer_path.as_ref() {
            let email = SendableEmail::new(
                envelope.clone(),
                email_id.clone(),
                body.clone().into_bytes()
            );
            let mut transport = FileTransport::new(dir);
            transport.send(email)
                .map_err(|e| {
                    error!("File Mailer Error: {}", e);
                    ()
                })?;
        }

        if let Some(smtp_client) = self.smtp_client.as_ref() {
            let mut transport = smtp_client.clone().transport();
            block(move || {
                let email = SendableEmail::new(
                    envelope,
                    email_id,
                    body.into_bytes()
                );
                transport.send(email)
            })
                .await
                .map_err(|e| {
                    error!("Could not send mail over SMTP: {}", e);
                    ()
                })?;
        }

        Ok(())
    }
}
