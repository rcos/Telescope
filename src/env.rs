use clap::{App, Arg, ArgGroup};
use std::env;
use std::process::exit;
use std::path::Path;

const LOG_LEVEL_ENV_VAR: &'static str = "LOG_LEVEL";
const TLS_CERT_FILE_ENV_VAR: &'static str = "CERT_FILE";
const TLS_PRIV_KEY_FILE_ENV_VAR: &'static str = "PRIV_KEY_FILE";
const DATABASE_URL_ENV_VAR: &'static str = "DATABASE_URL";
const BINDING_ENV_VAR: &'static str = "BIND_TO";
const SMTP_SENDER_NAME_ENV_VAR: &'static str = "SMTP_SENDER_NAME";
const SMTP_USERNAME_ENV_VAR: &'static str = "SMTP_USERNAME";
const SMTP_PASSWORD_ENV_VAR: &'static str = "SMTP_PASSWORD";
const SMTP_HOST_ENV_VAR: &'static str = "SMTP_HOST";
const SMTP_PORT_ENV_VAR: &'static str = "SMTP_PORT";
const SYSADMIN_EMAIL_ENV_VAR: &'static str = "ADMIN_EMAIL";
const SYSADMIN_PASSWORD_ENV_VAR: &'static str = "ADMIN_PASSWORD";

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

/// Stores the configuration of the telescope server. An instance of this is created and stored in
/// a lazy static before the server is launched.
#[derive(Debug, Serialize)]
pub struct Config {
    pub tls_cert_file: String,
    pub tls_key_file: String,
    pub bind_to: String,
    pub db_url: String,

    /*
    /// Domain. For development this will be 0.0.0.0.
    /// For production, it will likely be rcos.io.
    pub domain: Url,
    */

    pub email_config: EmailConfig,

    // Sysadmin info (email, password)
    pub sysadmin: Option<(String, String)>,
}

lazy_static! {
    /// Global web server configuration.
    pub static ref CONFIG: Config = cli();
}

/// Validate that the string has the path of a file that we can see.
fn is_file(s: String) -> Result<(), String> {
    let p = Path::new(&s);
    if p.is_file() {
        Ok(())
    }
    else if !p.exists() {
        Err(format!("{} does not exist.", p.display()))
    }
    else if p.is_dir() {
        Err(format!("{} is a directory.", p.display()))
    }
    else {
        Err(format!("{} is not a file.", p.display()))
    }
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

    let matches = App::new("telescope")
        .about("Telescope: the RCOS webapp.")
        // use the authors specified in Cargo.toml at compile time.
        .author(env!("CARGO_PKG_AUTHORS").replace(",", "\n").as_str())
        // use the version specified in Cargo.toml at compile time.
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("TLS_CERT_FILE")
                .long("tls-cert")
                .help("TLS/SSL certificate file. This is passed to OpenSSL.")
                .env(TLS_CERT_FILE_ENV_VAR)
                .takes_value(true)
                .default_value("tls-ssl/certificate.pem")
                .validator(is_file)
        )
        .arg(
            Arg::with_name("TLS_PRIV_KEY_FILE")
                .env(TLS_PRIV_KEY_FILE_ENV_VAR)
                .long("tls-key")
                .help("TLS/SSL private key file. This is passed to OpenSSL.")
                .takes_value(true)
                .default_value("tls-ssl/private-key.pem")
                .validator(is_file)
        )
        .arg(
            Arg::with_name("LOG_LEVEL")
                .help("Set the log level (or verbosity). \
                    See https://docs.rs/env_logger/0.7.1/env_logger/ for reference.")
                .env(LOG_LEVEL_ENV_VAR)
                .takes_value(true)
                .long("log-level")
                .short("v")
                .default_value("info"),
        )
        .arg(
            Arg::with_name("BIND_TO")
                .env(BINDING_ENV_VAR)
                .takes_value(true)
                .short("B")
                .long("bind-to")
                .help("Specify where to bind the web server.")
                .required_unless_one(&["DEVELOPMENT", "PRODUCTION"]),
        )
        .arg(
            Arg::with_name(DATABASE_URL_ENV_VAR)
                .takes_value(true)
                .short("D")
                .long("database-url")
                .help("Database URL passed to diesel.")
                .env(DATABASE_URL_ENV_VAR),
        )
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
        .arg(
            Arg::with_name("PRODUCTION")
                .help("\
                    Set web server to bind to localhost:443 (the standard https port).")
                .long("production")
                .short("p")
        )
        .arg(
            Arg::with_name("DEVELOPMENT")
                .help("\
                    Set web server to bind to localhost:8443 (testing port). \
                    Generate an admin account unless otherwise specified. \
                    Print server generated emails to the standard output unless \
                        otherwise specified")
                .long("development")
                .short("d")
        )
        .get_matches();

    // init logger
    env::set_var(LOG_LEVEL_ENV_VAR, matches.value_of("LOG_LEVEL").unwrap());
    env_logger::init_from_env(LOG_LEVEL_ENV_VAR);

    Config {
        tls_cert_file: matches.value_of("TLS_CERT_FILE").unwrap().to_owned(),
        tls_key_file: matches.value_of("TLS_PRIV_KEY_FILE").unwrap().to_owned(),
        bind_to: if matches.is_present("DEVELOPMENT") {
            Some("localhost:8443")
        } else if matches.is_present("PRODUCTION") {
            Some("localhost:443")
        } else {
            None
        }
        .or(matches.value_of("BIND_TO"))
        .unwrap()
        .to_owned(),
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
}

fn required(opt: Option<&str>, env: &'static str) -> String {
    opt
        .ok_or_else(|| {
            error!("{} must be specified.", env);
            exit(exitcode::NOINPUT)
        })
        .unwrap()
        .to_owned()
}

fn env_only_required(env_var: &'static str, name: &str) -> String {
    env::var(env_var)
        .map_err(|_| {
            error!("{} must be specified in .env file using {}.", name, env_var);
            exit(exitcode::NOINPUT)
        })
        .unwrap()
}