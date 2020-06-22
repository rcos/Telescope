#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

// #[macro_use]
// extern crate diesel;

mod env;
use crate::env::{Config, CONFIG};

mod web;
use web::*;

mod templates;

use crate::web::app_data::AppData;
use actix_files as afs;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_session::CookieSession;
use actix_web::cookie::SameSite;
use actix_web::{middleware, web as aweb, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use rand::rngs::OsRng;
use rand::Rng;
use std::process::exit;
use std::time::Duration;
use actix_web::web::{get, post};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // set up logger and global web server configuration.
    env::init();
    let config: &Config = &*CONFIG;

    // from example at https://actix.rs/docs/http2/
    // to generate a self-signed certificate and private key for testing, use
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout tls-ssl/private-key.pem -out tls-ssl/certificate.pem -days 365`
    let mut tls_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .map_err(|e| {
            error!("Could not initialize TLS/SSL builder: {}", e);
            exit(exitcode::SOFTWARE)
        })
        .unwrap();
    tls_builder
        .set_private_key_file(&config.tls_key_file, SslFiletype::PEM)
        .map_err(|e| {
            error!(
                "Could not read TLS/SSL private key at {}: {}",
                config.tls_key_file, e
            );
            exit(exitcode::NOINPUT)
        })
        .unwrap();
    tls_builder
        .set_certificate_chain_file(&config.tls_cert_file)
        .map_err(|e| {
            error!(
                "Could not read TLS/SSL certificate chain file at {}: {}",
                config.tls_cert_file, e
            );
            exit(exitcode::NOINPUT)
        })
        .unwrap();

    // register handlebars templates
    let mut template_registry = Handlebars::new();
    template_registry
        .register_templates_directory(".hbs", "templates")
        .map_err(|e| {
            error!("Failed to properly register handlebars templates: {}", e);
            exit(1)
        })
        .unwrap();
    // Use handlebars strict mode so that we get an error when we try to render a
    // non-existent field
    template_registry.set_strict_mode(true);
    info!("Handlebars templates registered.");

    // generate a random key to encrypt cookies.
    let mut rng = OsRng::default();
    let mut cookie_key = [0u8; 32];
    rng.fill(&mut cookie_key);

    // memory store for rate limiting.
    let ratelimit_memstore = MemoryStore::new();

    // Create appdata (handlebars registry storage object currently)
    let app_data = AppData::new(template_registry);

    HttpServer::new(move || {
        App::new()
            .data(app_data.clone())
            .wrap(
                CookieSession::signed(&cookie_key)
                    .same_site(SameSite::Strict)
                    .http_only(true),
            )
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(ratelimit_memstore.clone()).start())
                    // rate limit: 100 requests max per minute
                    .with_interval(Duration::from_secs(60))
                    .with_max_requests(100),
            )
            .wrap(middleware::Logger::default())
            .service(afs::Files::new("/static", "static"))
            .route("/", get().to(index::index_service))
            .route("/sponsors", get().to(sponsors::sponsors_page))
            .route("/auth", post().to(auth::auth_service))
            .default_service(aweb::route().to(|| HttpResponse::NotFound()))
    })
    .bind_openssl(config.bind_to.clone(), tls_builder)
    .map_err(|e| {
        error!("Could not bind to {}: {}", config.bind_to, e);
        exit(e.raw_os_error().unwrap_or(1))
    })
    .unwrap()
    .run()
    .await
}
