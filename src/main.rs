
#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate property;

#[macro_use]
extern crate lazy_static;

mod env;
use crate::env::{
    Config,
    CONFIG
};

use actix_files as afs;
use handlebars::Handlebars;
use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype};
use std::process::exit;
use actix_web::{HttpServer, App, middleware, HttpResponse};
use actix_web::web::route;


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
            error!("Could not read TLS/SSL private key at {}: {}", config.tls_key_file, e);
            exit(exitcode::NOINPUT)
        })
        .unwrap();
    tls_builder.set_certificate_chain_file(&config.tls_cert_file)
        .map_err(|e| {
            error!("Could not read TLS/SSL certificate chain file at {}: {}", config.tls_cert_file, e);
            exit(exitcode::NOINPUT)
        })
        .unwrap();

    // register handlebars templates
    let mut template_registry = Handlebars::new();
    template_registry.register_templates_directory(".hbs", "templates")
        .map_err(|e| {
            error!("Failed to properly register handlebars templates: {}", e);
            exit(1)
        })
        .unwrap();
    info!("Handlebars templates registered.");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(afs::Files::new("/static/", "static"))
            .default_service(route().to(|| HttpResponse::NotFound()))

    })
        //.bind("localhost:8088")
        .maxconn(1000)
        .bind_openssl(config.bind_to.clone(), tls_builder)
        .map_err(|e| {
            error!("Could not bind to {}: {}", config.bind_to, e);
            exit(e.raw_os_error().unwrap_or(1))
        })
        .unwrap()
        .run()
        .await
}
