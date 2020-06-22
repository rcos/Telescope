use clap::{App, Arg, ArgGroup};
use std::env;

const LOG_LEVEL_ENV_VAR: &'static str = "LOG_LEVEL";
const TLS_CERT_FILE_ENV_VAR: &'static str = "CERT_FILE";
const TLS_PRIV_KEY_FILE_ENV_VAR: &'static str = "PRIV_KEY_FILE";
const DATABASE_URL_ENV_VAR: &'static str = "DATABASE_URL";

/// Stores the configuration of the telescope server. An instance of this is created and stored in
/// a lazy static before the server is launched.
#[derive(Debug)]
pub struct Config {
    pub tls_cert_file: String,
    pub tls_key_file: String,
    pub bind_to: String,
    pub db_url: String,
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
    info!("TLS/SSL certificate location: {}", cfg.tls_cert_file);
    info!("TLS/SSL private key location: {}", cfg.tls_key_file);
}

/// Digest and handle arguments from the command line. Read arguments from environment
/// variables where necessary. Construct and return the configuration specified.
/// Initializes logging and returns config.
fn cli() -> Config {
    // set env vars from a ".env" file if available.
    dotenv::dotenv().ok();

    let matches = App::new("telescope")
        .about("Telescope: the RCOS webapp.")
        .author(env!("CARGO_PKG_AUTHORS").replace(",", "\n").as_str()) // use the authors specified in Cargo.toml at compile time.
        .version(env!("CARGO_PKG_VERSION")) // use the version specified in Cargo.toml at compile time.
        .arg(
            Arg::with_name("TLS_CERT_FILE")
                .long("tls-cert")
                .help("TLS/SSL certificate file. This is passed to OpenSSL.")
                .env(TLS_CERT_FILE_ENV_VAR)
                .takes_value(true)
                .default_value("tls-ssl/certificate.pem"),
        )
        .arg(
            Arg::with_name("TLS_PRIV_KEY_FILE")
                .env(TLS_PRIV_KEY_FILE_ENV_VAR)
                .long("tls-key")
                .help("TLS/SSL private key file. This is passed to OpenSSL.")
                .takes_value(true)
                .default_value("tls-ssl/private-key.pem"),
        )
        .arg(
            Arg::with_name("LOG_LEVEL")
                .help("Set the log level (or verbosity).")
                .env(LOG_LEVEL_ENV_VAR)
                .takes_value(true)
                .long("log-level")
                .short("v")
                .possible_values(&["trace", "debug", "info", "warn", "error"])
                .default_value("info"),
        )
        .arg(
            Arg::with_name("BIND_TO")
                .takes_value(true)
                .short("B")
                .long("bind-to")
                .help("Specify where to bind the web server."),
        )
        .arg(
            Arg::with_name("DATABASE_URL")
                .takes_value(true)
                .short("D")
                .long("database-url")
                .help("Database URL passed to Diesel")
                .env(DATABASE_URL_ENV_VAR)
        )
        .arg(
            Arg::with_name("PRODUCTION")
                .help("Set web server to bind to localhost:443 (the standard https port).")
                .long("production"),
        )
        .arg(
            Arg::with_name("DEVELOPMENT")
                .help("Set web server to bind to localhost:8443 (testing port).")
                .long("development"),
        )
        .group(
            ArgGroup::with_name("BINDING")
                .args(&["DEVELOPMENT", "PRODUCTION", "BIND_TO"])
                .required(true),
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
        db_url: matches.value_of("DATABASE_URL").unwrap().to_owned(),
    }
}
