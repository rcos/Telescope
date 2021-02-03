#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate derive_more;

// mod web;
mod env;
mod templates;
mod error;
mod models;
mod app_data;
mod web;

use app_data::AppData;
use crate::{
    env::{ConcreteConfig, CONFIG},
    templates::static_pages::{
        index::LandingPage,
        projects::ProjectsPage,
        sponsors::SponsorsPage,
        StaticPage
    },
};
use std::sync::Arc;
use actix::prelude::*;
use actix_files as afs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{http::Uri, middleware, web as aweb, web::get, App, HttpServer};
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use openssl::ssl::{SslAcceptor, SslMethod};
use rand::{rngs::OsRng, Rng};
use std::process::exit;
use crate::error::TelescopeError;

fn main() -> std::io::Result<()> {
    // set up logger and global web server configuration.
    env::init();
    let config: &ConcreteConfig = &*CONFIG;

    // Create the actix runtime.
    let sys = System::new("telescope");

    // from example at https://actix.rs/docs/http2/
    // to generate a self-signed certificate and private key for testing, use
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout tls-ssl/private-key.pem -out tls-ssl/certificate.pem -days 365`
    let mut tls_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Could not create SSL Acceptor.");
    config.tls_config.init_tls_acceptor(&mut tls_builder);

    // generate a random key to encrypt cookies.
    let cookie_key = OsRng::default().gen::<[u8; 32]>();

    // Get ports for redirecting HTTP to HTTPS
    let http_port = config
        .bind_http
        .as_str()
        .parse::<Uri>()
        .expect("Invalid HTTP (http/1) URI")
        .port()
        .map(|p| format!(":{}", p.as_str()));

    let https_port = config
        .bind_https
        .as_str()
        .parse::<Uri>()
        .expect("Invalid HTTPS (http/2) URI")
        .port()
        .map(|p| format!(":{}", p.as_str()));

    // Build redirect middleware.
    let mut redirect_middleware: RedirectSchemeBuilder = RedirectSchemeBuilder::new();

    redirect_middleware
        .enable(true)
        .http_to_https(true)
        .permanent(true);

    if http_port.is_some() && https_port.is_some() {
        redirect_middleware.replacements(&[(http_port.unwrap(), https_port.unwrap())]);
    }

    HttpServer::new(move || {
        App::new()
            // Compression middleware
            .wrap(middleware::Compress::default())
            // Identity and authentication middleware.
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(&cookie_key)
            //         .name(cookies::AUTH_TOKEN)
            //         .secure(true)
            //         // Cookies / sessions expire after 24 hours
            //         .max_age_time(time::Duration::hours(24)),
            // ))
            // Redirect to HTTP -> HTTPS middleware.
            .wrap(redirect_middleware.build())
            // logger middleware
            .wrap(middleware::Logger::default())
            // register Services
            .configure(web::services::register)
            // static files service
            .service(afs::Files::new("/static", "static"))
            .route("/", get().to(LandingPage::handle))
            .route("/projects", get().to(ProjectsPage::handle))
            .route("/sponsors", get().to(SponsorsPage::handle))
            // .default_service(aweb::route().to(TelescopeError::PageNotFound))
    })
    .bind(config.bind_http.clone())
    .expect("Could not bind HTTP/1 (HTTP)")
    .bind_openssl(config.bind_https.clone(), tls_builder)
    .expect("Could not bind HTTP/2 (HTTPS)")
    .run();

    // Start the actix runtime.
    sys.run()
}
