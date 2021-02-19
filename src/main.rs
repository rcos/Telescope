#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate async_trait;

use actix::prelude::*;
use actix_files as afs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, http::Uri, HttpServer, middleware, web as aweb, web::get};
use actix_web::cookie::SameSite;
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use openssl::ssl::{SslAcceptor, SslMethod};
use rand::Rng;
use rand::rngs::OsRng;

use crate::{
    env::{ConcreteConfig, CONFIG},
    templates::static_pages::{
        projects::ProjectsPage, sponsors::SponsorsPage, StaticPage,
    },
};
use crate::web::csrf::CsrfJanitor;

mod app_data;
mod env;
mod error;
mod templates;
mod web;

fn main() -> std::io::Result<()> {
    // set up logger and global web server configuration.
    env::init();
    let config: &ConcreteConfig = &*CONFIG;

    // Create the actix runtime.
    let sys = System::new("telescope");

    // Start global CSRF token janitor.
    CsrfJanitor.start();

    // from example at https://actix.rs/docs/http2/
    // to generate a self-signed certificate and private key for testing, use
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout tls-ssl/private-key.pem -out tls-ssl/certificate.pem -days 365`
    let mut tls_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Could not create SSL Acceptor.");
    config.tls_config.init_tls_acceptor(&mut tls_builder);

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

    // Setup identity middleware.
    // Create secure random sequence to encrypt cookie identities.
    let cookie_key: [u8; 32] = OsRng::default().gen::<[u8; 32]>();

    // Construct and start main server instance.
    HttpServer::new(move || {
        // Create cookie policy.
        let cookie_policy = CookieIdentityPolicy::new(&cookie_key)
            // Transmit cookies over HTTPS only.
            .secure(true)
            .name("telescope_auth")
            .same_site(SameSite::Strict)
            // Cookies expire after a day.
            .max_age_time(time::Duration::days(1));

        App::new()
            // Middleware to render telescope errors into pages
            .wrap(web::error_rendering_middleware::TelescopeErrorHandler)
            // Redirect to HTTP -> HTTPS middleware.
            .wrap(redirect_middleware.build())
            // Cookie Identity middleware.
            .wrap(IdentityService::new(cookie_policy))
            // Compression middleware.
            .wrap(middleware::Compress::default())
            // Logger middleware
            .wrap(middleware::Logger::default())
            // register Services
            .configure(web::services::register)
            // static files service
            .service(afs::Files::new("/static", "static"))
            .route("/projects", get().to(ProjectsPage::handle))
            .route("/sponsors", get().to(SponsorsPage::handle))
            .default_service(aweb::to(web::services::not_found::not_found))
    })
    .bind(config.bind_http.clone())
    .expect("Could not bind HTTP/1 (HTTP)")
    .bind_openssl(config.bind_https.clone(), tls_builder)
    .expect("Could not bind HTTP/2 (HTTPS)")
    .run();

    // Start the actix runtime.
    sys.run()
}
