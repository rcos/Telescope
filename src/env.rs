use std::env;
use structopt::StructOpt;

const SMTP_SENDER_NAME_ENV_VAR: &'static str = "SMTP_SENDER_NAME";
const SMTP_USERNAME_ENV_VAR: &'static str = "SMTP_USERNAME";
const SMTP_PASSWORD_ENV_VAR: &'static str = "SMTP_PASSWORD";
const SMTP_HOST_ENV_VAR: &'static str = "SMTP_HOST";
const SMTP_PORT_ENV_VAR: &'static str = "SMTP_PORT";

/// Stores the configuration of the server's email
#[derive(Debug, Serialize)]
pub enum EmailConfig {
    StubConfig {
        sender_name: String,
        dummy_username: String,
        dummy_host: String
    },
    FileTransportConfig {
        temp_dir: String,
        sender_name: String,
        dummy_username: String,
        dummy_host: String,
    },
    LiveConfig {
        port: u16,
        host: String,
        username: String,
        password: String,
        sender_name: String
    }
}

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
        short = "B",
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
    #[structopt(short = "U", long = "domain", env)]
    pub domain: String,

    /// The URL the Postgres Database is running at.
    /// This is passed directly to diesel.
    #[structopt(short = "D", long = "database-url", env)]
    pub db_url: String,

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

    /// Use Development profile. This sets the following defaults:
    ///
    /// BIND_TO: localhost:8443
    #[structopt(
        short = "d",
        long = "development",
        conflicts_with("PRODUCTION")
    )]
    development: bool,

    /// Use Production profile. This sets the following defaults:
    ///
    /// BIND_TO: localhost:443
    #[structopt(
        short = "p",
        long = "production",
        conflicts_with("DEVELOPMENT")
    )]
    production: bool,
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

    let global_tmp_dir = env::temp_dir().to_str().unwrap();
        /*
        .arg(
            Arg::with_name("EMAIL_TRANSPORT")
                .takes_value(true)
                .possible_values(&[
                    "stub", "temp-dir", "SMTP"
                ])
                .use_delimiter(true)
                .short("E")
                .long("email-transport")
                .required_unless("DEVELOPMENT")
                .help(format!("Stub prints emails to standard output, temp-dir saves emails in \
                    files in {}, SMTP sends emails using an SMTP server.",
                              env::temp_dir().display()).as_str())
                .default_value_if("DEVELOPMENT", None, "stub")

        )
        .arg(
            Arg::with_name(SMTP_SENDER_NAME_ENV_VAR)
                .takes_value(true)
                .empty_values(false)
                .long("email-sender-name")
                .help("Name associated with account verification emails.")
                .env(SMTP_SENDER_NAME_ENV_VAR)
                .default_value("RCOS Telescope")
        )
        .arg(
            Arg::with_name(SMTP_USERNAME_ENV_VAR)
                .takes_value(true)
                .long("smtp-username")
                .help("Username to access SMTP email server.")
                .env(SMTP_USERNAME_ENV_VAR)
        )
        .arg(
            Arg::with_name(SMTP_PASSWORD_ENV_VAR)
                .takes_value(true)
                .long("smtp-pass")
                .help("Password to access SMTP email server.")
                .env(SMTP_PASSWORD_ENV_VAR)
        )
        .arg(
            Arg::with_name(SMTP_HOST_ENV_VAR)
                .takes_value(true)
                .long("smtp-host")
                .env(SMTP_HOST_ENV_VAR)
                .help("SMTP host to use.")
        )
        .arg(
            Arg::with_name(SMTP_PORT_ENV_VAR)
                .takes_value(true)
                .validator(|e| e.parse::<u16>()
                    .map_err(|e| e.to_string())
                    .map(|_| ())
                )
                .help("SMTP email server port")
                .default_value("25")
                .long("smtp-port")
                .env(SMTP_PORT_ENV_VAR)
        )
        */
    /*
    .arg(
            Arg::with_name("CREATE_SYSADMIN")
                .help(&format!("Create a sysadmin account using the email and \
                    password specified in the .env file using {} for the email \
                    and {} for the password.",
                              SYSADMIN_EMAIL_ENV_VAR, SYSADMIN_PASSWORD_ENV_VAR))
                .long("create-sysadmin-account")
                .short("I")
                .takes_value(true)
                .possible_values(&["true", "false"])
        )

     */
    let config: Config = Config::from_args();

    env_logger::builder()
        .parse_filters(&config.log_level)
        .init();

    return config;

    /*
    Config {
        tls_cert_file: matches.value_of("TLS_CERT_FILE").unwrap().to_owned(),
        tls_key_file: matches.value_of("TLS_PRIV_KEY_FILE").unwrap().to_owned(),
        db_url: required(matches.value_of(DATABASE_URL_ENV_VAR), DATABASE_URL_ENV_VAR),
        smtp_sender_name: matches.value_of(SMTP_SENDER_NAME_ENV_VAR).unwrap().to_owned(),
        smtp_username: required(matches.value_of(SMTP_USERNAME_ENV_VAR), SMTP_USERNAME_ENV_VAR),
        smtp_password: required(matches.value_of(SMTP_PASSWORD_ENV_VAR), SMTP_PASSWORD_ENV_VAR),
        smtp_host: matches.value_of(SMTP_HOST_ENV_VAR).map(|s| s.to_owned()),
        smtp_port: matches.value_of(SMTP_PORT_ENV_VAR)
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap(),
        sysadmin: {
            matches.value_of("CREATE_SYSADMIN")
                .or_else(|| {
                    if matches.is_present("DEVELOPMENT") {
                        Some("true")
                    } else {
                        None
                    }
                })
                .map(|v| v == "true")
                .map(|b| {
                    if b {
                        Some((
                            env_only_required(
                                SYSADMIN_EMAIL_ENV_VAR,
                                "System admin login email"
                            ),
                            env_only_required(
                                SYSADMIN_PASSWORD_ENV_VAR,
                                "System admin password"
                            )
                        ))
                    } else {
                        None
                    }
                })
                .flatten()
        }
    }
     */
}