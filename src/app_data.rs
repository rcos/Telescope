use crate::{
    env::{ConcreteConfig, CONFIG},
    templates::Template,
    util::DbConnection,
};
use actix_web::web::block;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use handlebars::{Handlebars, RenderError};
use lettre::{
    stub::StubTransport, FileTransport, SendableEmail, SmtpClient, SmtpTransport, Transport,
};
use lettre_email::Mailbox;
use std::{path::PathBuf, sync::Arc};
use crate::error::TelescopeError;

lazy_static!{
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
    /// Database connection pool
    db_connection_pool: Pool<ConnectionManager<PgConnection>>,
    /// SMTP Mailer client config (if in use).
    smtp_client: Option<SmtpClient>,
    /// Path for file mailer if in use.
    file_mailer_path: Option<PathBuf>,
    /// Should mail stubs be created?
    use_stub_mailer: bool,
    /// The email sender address.
    mail_sender: Mailbox,
}

impl AppData {
    /// Create new App Data object using the global static config.
    fn new() -> Self {
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
                address: config.email_config.address.to_string(),
            },
        }
    }

    /// Get an [`Arc`] reference to the global, lazily generated app-data.
    pub fn global() -> Arc<AppData> {
        APP_DATA.clone()
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
        self.file_mailer_path
            .as_ref()
            .map(|pb| FileTransport::new(pb.as_path()))
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
    where
        M: Into<SendableEmail> + Clone + Send + Sync + 'static,
    {
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
            })
            .await;

            if let Err(e) = result {
                error!("Could not send mail over SMTP: {}", e);
                return Err(());
            } else {
                let response = result.unwrap();
                if !response.is_positive() {
                    return Err(());
                }
            }
        }

        Ok(())
    }

    /// Get an [`Arc`] reference to the template registry.
    pub fn get_handlebars_registry(&self) -> Arc<Handlebars<'static>> {
        self.template_registry.clone()
    }

    /// Render a handlebars template using this object's registry.
    pub fn render_template(&self, template: &Template) -> Result<String, RenderError> {
        self.get_handlebars_registry().render(template.handlebars_file, &template)
    }

    /// Asynchronously get a database connection.
    pub async fn get_db_conn(&self) -> Result<DbConnection, TelescopeError> {
        // Clone the connection pool reference.
        let db_conn_pool: Pool<ConnectionManager<PgConnection>> = self.clone_db_conn_pool();

        // Asynchronously try to retrieve a connection from the pool.
        // This may block/take some time, but should timeout instead of waiting indefinitely.
        block::<_, DbConnection, _>(move || {
            db_conn_pool.get()
        }).await.into()
    }

}
