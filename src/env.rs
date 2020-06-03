
use std::env;
use clap::{App, Arg};

const LOG_LEVEL_ENV_VAR: &'static str = "LOG_LEVEL";

/// Initialize the working environment. Read args from
/// the bash environment, the command line, and when not available, use defaults.
///
/// Initialize logging.
pub fn init() {
    cli();
    // if logging level is not set, set default.
    if env::var(LOG_LEVEL_ENV_VAR).is_err() {
        env::set_var(LOG_LEVEL_ENV_VAR, "info");
    }

    // init logger
    env_logger::init_from_env(LOG_LEVEL_ENV_VAR);
    info!("Starting up...");
    info!("telescope {}", env!("CARGO_PKG_VERSION"))

}

/// Digest and handle arguments from the command line.
fn cli() {
    App::new("telescope")
        .about("Telescope: the RCOS webapp.")
        .author(env!("CARGO_PKG_AUTHORS").replace(",", "\n").as_str()) // use the authors specified in Cargo.toml at compile time.
        .version(env!("CARGO_PKG_VERSION"))// use the version specified in Cargo.toml at compile time.
        .arg(Arg::with_name("TLS_CERT_FILE")
            .long("tls-cert")
            .help("TLS/SSL certificate file. This is passed to OpenSSL.")
            .takes_value(true))
        .arg(Arg::with_name("TLS_PRIV_KEY_FILE")
            .long("tls-key")
            .help("TLS/SSL private key file. This is passed to OpenSSL.")
            .takes_value(true))
        .arg(Arg::with_name("LOG_LEVEL")
            .help("Set the log level (or verbosity).")
            .env(LOG_LEVEL_ENV_VAR)
            .takes_value(true)
            .long("log-level")
            .short("v")
            .possible_values(&["trace", "debug", "info", "warn", "error"])
            .default_value("info")
            )
        .arg(Arg::with_name("BIND_TO")
            .takes_value(true)
            .short("B")
            .long("bind-to")
            .help("Specify where to bind the web server."))
        .get_matches();
}