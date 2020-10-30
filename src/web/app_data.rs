use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};
use handlebars::Handlebars;
use std::{
    sync::Arc,
    path::PathBuf
};
use lettre::{SmtpClient, SendableEmail, Transport, FileTransport, stub::StubTransport, SmtpTransport};
use crate::env::{
    CONFIG,
    ConcreteConfig
};
use actix_web::web::block;
use lettre_email::Mailbox;

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
    pub mail_sender: Mailbox,
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
            mail_sender: Mailbox {
                name: config.email_config.name.clone(),
                address: config.email_config.address.to_string()
            }
        }
    }

    /// Get a clone of the database connection pool (which internally uses an Arc,
    /// so this is just a reference copy).
    pub fn clone_db_conn_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        self.db_connection_pool.clone()
    }

    /// Get an SMTP transport if available.
    fn get_smtp_transport(&self) -> Option<SmtpTransport> {
        self.smtp_client.clone().map(|client| client.transport())
    }

    /// Get a file based mail transport if available.
    fn get_file_mail_transport(&self) -> Option<FileTransport> {
        self.file_mailer_path.as_ref().map(|pb| FileTransport::new(pb.as_path()))
    }

    /// Get a stub based mail transporter if available.
    fn get_stub_transport(&self) -> Option<StubTransport> {
        if self.use_stub_mailer {
            Some(StubTransport::new_positive())
        } else {
            None
        }
    }

    /// Send an email over all available mailer interfaces.
    /// Any errors caught while mailing will be logged and then an `Err`
    /// will be returned.
    pub async fn send_mail<M>(&self, mail: M) -> Result<(), ()>
    where M: Into<SendableEmail> + Clone + Send + Sync + 'static {
        if let Some(mut t) = self.get_stub_transport() {
            t.send(mail.clone().into())?;
        }

        if let Some(mut t) = self.get_file_mail_transport() {
            t.send(mail.clone().into()).map_err(|e| {
                error!("Error while mailing to local file system: {}", e);
                ()
            })?;
        }

        if let Some(mut t) = self.get_smtp_transport() {
            // Use blocking call because I am honestly not sure if this call
            // will or won't block, but it seems to establish a connection to the
            // SMTP server so I'm assuming it does.

            let result = block(move || {
                let res = t.send(mail.into());
                t.close();
                res
            }).await;

            if let Err(e) = result {
                error!("Could not send mail over SMTP: {}", e);
                return Err(());
            } else {
                let response = result.unwrap();
                info!("Received SMTP Response code {}: {:?}", response.code, response.message);
                if !response.is_positive() {
                    return Err(());
                }
            }
        }

        Ok(())
    }
}
