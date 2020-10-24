use std::env;
use std::path::PathBuf;
use structopt::clap::ArgGroup;
use structopt::StructOpt;
use lettre::SmtpClient;

// The name, about, version, and authors are given by cargo.
/// Stores the configuration of the telescope server. An instance of this is created and stored in
/// a lazy static before the server is launched.
#[derive(Debug, Serialize, StructOpt)]
#[structopt(about = "The RCOS webapp", rename_all = "screaming-snake")]
pub struct Config {
    /// The TLS certificate. See the readme for instructions to generate your
    /// own.
    #[structopt(long = "tls-cert-file", default_value = "tls-ssl/certificate.pem", env)]
    pub tls_cert_file: String,

    /// The TLS private key file. See the readme for instructions to generate
    /// your own.
    #[structopt(long = "tls-priv-key", default_value = "tls-ssl/private-key.pem", env)]
    pub tls_priv_key_file: String,

    /// Set the log level (or verbosity).
    /// See https://docs.rs/env_logger/0.7.1/env_logger/ for reference.
    #[structopt(
        short = "v",
        long = "log-level",
        default_value = "info",
        default_value_if("DEVELOPMENT", None, "info,telescope=trace"),
        env
    )]
    log_level: String,

    /// The URL to bind the running server to.
    #[structopt(
        short = "b",
        long = "bind-to",
        env,
        default_value_ifs(&[
            ("DEVELOPMENT", None, "localhost:8443"),
            ("PRODUCTION", None, "localhost:443")
        ]),
    )]
    pub bind_to: String,

    /// The domain that telescope is running at.
    /// This is used to redirect callbacks to after going offsite for
    /// authentication. This is also used to generate confirmation links
    /// that get emailed to users.
    #[structopt(
        long = "domain",
        env,
        default_value_if("DEVELOPMENT", None, "https://localhost:8443"),
    )]
    pub domain: String,

    /// The URL the Postgres Database is running at.
    /// This is passed directly to diesel.
    #[structopt(short = "D", long = "database-url", env)]
    pub database_url: String,

    /// Create a sysadmin account on startup.
    #[structopt(
        short = "S",
        long = "create-sysadmin",
        requires_all(&["SYSADMIN_EMAIL", "SYSADMIN_PASSWORD"])
    )]
    pub create_sysadmin: bool,

    /// The email to seed the sysadmin account with.
    #[structopt(long = "sysadmin-email", env = "ADMIN_EMAIL")]
    pub sysadmin_email: Option<String>,

    /// The password to seed the sysadmin account with.
    #[structopt(long = "sysadmin-pass", env = "ADMIN_PASSWORD")]
    pub sysadmin_password: Option<String>,

    /// Set how sending emails to users is handled.
    /// There are three options here:
    /// - Stub: Print generated emails to the standard output
    /// - File: Save emails to a directory in the file system.
    /// - SMTP: Log into an email server over smtp and send the generated
    ///     emails to their recipients.
    #[structopt(
        short = "e",
        long = "email",
        env = "EMAIL",
        default_value_if("DEVELOPMENT", None, "stub"),
        possible_values(&[
            "stub",
            "file",
            "smtp"
        ]),
        requires_all(&[
            "EMAIL_SENDER_NAME",
            "EMAIL_USER",
            "EMAIL_HOST",
        ]),
        requires_ifs(&[
            ("smtp", "SMTP_PASSWORD"),
            ("smtp", "SMTP_PORT"),
            // name of manually created group later
            ("file", "EMAIL_FILE_OPTION"),
        ])
    )]
    email_senders: Vec<String>,

    /// Display name of the sender of system emails.
    #[structopt(
        long = "email-sender-name",
        env = "EMAIL_SENDER",
        default_value_if("DEVELOPMENT", None, "RCOS Telescope")
    )]
    email_sender_name: Option<String>,

    /// The username in the server email address
    /// (the part before the @ symbol).
    #[structopt(
        long = "email-user",
        env,
        default_value_if("DEVELOPMENT", None, "telescope")
    )]
    email_user: Option<String>,

    // email_file_dir and email_use_temp_dir are grouped together manually using
    // clap in the cli function. This just makes specifying the requirement rules
    // easier.
    /// The directory to save emails to when saving emails to the
    /// filesystem.
    #[structopt(long = "email-file-dir", env)]
    email_file_dir: Option<PathBuf>,

    /// Default the email save directory to the system temp directory.
    /// The location of this will be operating system dependent.
    #[structopt(long = "email-sys-temp", env)]
    email_use_temp_dir: bool,

    /// The host in the server email address.
    /// (the part after the @ symbol).
    #[structopt(
        long = "email-host",
        env,
        default_value_if("DEVELOPMENT", None, "rcos.io")
    )]
    email_host: Option<String>,

    /// The host to log into via SMTP to send emails to users.
    #[structopt(long = "smtp-password", env)]
    smtp_password: Option<String>,

    /// The port of the SMTP server that this server uses to send mail.
    #[structopt(long = "smtp-port", env)]
    smtp_port: Option<u16>,

    /// Use Development profile. This sets the following defaults:
    ///
    /// BIND_TO: localhost:8443
    /// DOMAIN: https://localhost:8443
    /// LOG_LEVEL: info,telescope=trace
    /// EMAIL_SENDER: RCOS Telescope
    /// EMAIL: stub
    /// EMAIL_USER: telescope
    /// EMAIL_HOST: rcos.io
    #[structopt(short = "d", long = "development", conflicts_with("PRODUCTION"))]
    development: bool,

    /// Use Production profile. This sets the following defaults:
    ///
    /// BIND_TO: localhost:443
    #[structopt(short = "p", long = "production", conflicts_with("DEVELOPMENT"))]
    production: bool,
}

impl Config {
    /// Should the Stub email sender be used.
    pub fn use_stub_mailer(&self) -> bool {
        let stub_string = "stub".to_string();
        self.email_senders.contains(&stub_string)
    }

    /// Check if the user wants to use the file email transport
    /// and if so, check the specified path.
    pub fn get_file_mailer(&self) -> Option<PathBuf> {
        let sender_string = "file".to_string();
        if self.email_senders.contains(&sender_string) {
           self.email_file_dir.clone()
        } else {
            None
        }
    }

    /// Make an SMTP client if that mailer is to be used.
    pub fn make_smtp_mailer(&self) -> Option<SmtpClient> {
        // TODO: implement SMTP sender creation.
        unimplemented!()
    }
}

lazy_static! {
    /// Global web server configuration.
    pub static ref CONFIG: Config = cli();
}

/// After the global configuration is initialized, log it as info.
pub fn init() {
    let cfg: &Config = &*CONFIG;
    info!("Starting up...");
    info!("telescope {}", env!("CARGO_PKG_VERSION"));
    trace!("Config: \n{}", serde_json::to_string_pretty(cfg).unwrap());
}

/// Digest and handle arguments from the command line. Read arguments from environment
/// variables where necessary. Construct and return the configuration specified.
/// Initializes logging and returns config.
fn cli() -> Config {
    // set env vars from a ".env" file if available.
    dotenv::dotenv().ok();

    let mut config: Config = Config::from_clap(
        &Config::clap()
            .group(
                ArgGroup::with_name("EMAIL_FILE_OPTION")
                    .args(&["EMAIL_USE_TEMP_DIR", "EMAIL_FILE_DIR"]),
            )
            .get_matches(),
    );

    if config.email_use_temp_dir {
        config.email_file_dir = Some(env::temp_dir());
    }

    env_logger::builder()
        .parse_filters(&config.log_level)
        .init();

    return config;
}
