#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate diesel;

mod env;
use crate::env::{Config, CONFIG};

mod web;
use web::*;

mod templates;

mod schema;

mod models;

use crate::{
    templates::{
        static_pages::{index::LandingPage, sponsors::SponsorsPage, projects::ProjectsPage, developers::DevelopersPage},
        StaticPage,
    },
    web::app_data::AppData,
    models::{
        User,
        Email,
        password_requirements::{
            PasswordRequirements,
            MIN_LENGTH
        }
    },
};

use actix_files as afs;

use actix_identity::{CookieIdentityPolicy, IdentityService};

use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};

use actix_web::{
    middleware, web as aweb,
    web::{get, post},
    App, HttpServer,
};


use diesel::{r2d2::ConnectionManager, PgConnection, Connection, RunQueryDsl};

use rand::{rngs::OsRng, Rng};

use handlebars::Handlebars;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::process::exit;

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

    // Set up database connection pool.
    let manager = ConnectionManager::<PgConnection>::new(&config.db_url);
    let pool = diesel::r2d2::Pool::builder()
        // max 12 connections at once
        .max_size(12)
        // if a connection cannot be pulled from the pool in 20 seconds, timeout
        .connection_timeout(std::time::Duration::from_secs(20))
        .build(manager)
        .map_err(|e| {
            error!("Could not create database connection pool {}", e);
            exit(1);
        })
        .unwrap();
    info!("Created database connection pool.");

    if let Some((admin_email, admin_password)) = &config.sysadmin {
        let mut user: User = User::new("Telescope admin", admin_password)
            .map_err(|e: PasswordRequirements| {
                error!("Admin password {} failed to satisfy password requirements.", admin_password);
                if !e.not_common_password {
                    error!("Admin password {} is too common. Please choose a different password.", admin_password);
                }
                if !e.is_min_len {
                    error!("Admin password {} is too short. \
                        Please choose a password more than {} characters.", admin_password, MIN_LENGTH)
                }
                exit(exitcode::DATAERR)
            })
            .unwrap();
        let email = Email::new(user.id, admin_email)
            .unwrap_or_else(|| {
                error!("Admin email {} is not a valid email.", admin_email);
                exit(exitcode::DATAERR);
            });

        user.sysadmin = true;

        let conn = pool.get().unwrap();
        conn.transaction::<(), diesel::result::Error, _>(|| {
            use crate::schema::users::dsl::users;
            use crate::schema::emails::dsl::emails;

            diesel::insert_into(users)
                .values(user)
                .execute(&conn)?;

            diesel::insert_into(emails)
                .values(email)
                .execute(&conn)?;

            info!("Successfully added admin user");
            Ok(())
        })
            .map_err(|e| {
                error!("Could not add admin user to database: {}", e);
                exit(1)
            })
            .unwrap();
    }

    // Create appdata object.
    let app_data = AppData::new(template_registry, pool);

    // generate a random key to encrypt cookies.
    let cookie_key = OsRng::default().gen::<[u8; 32]>();

    // memory store for rate limiting.
    let ratelimit_memstore = MemoryStore::new();

    HttpServer::new(move || {
        App::new()
            .data(app_data.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookie_key)
                    .name(cookies::AUTH_TOKEN)
                    .secure(true)
                    // Cookies / sessions expire after 24 hours
                    .max_age_time(chrono::Duration::hours(24)),
            ))
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(ratelimit_memstore.clone()).start())
                    // rate limit: 100 requests max per minute
                    .with_interval(std::time::Duration::from_secs(60))
                    .with_max_requests(100),
            )
            .wrap(middleware::Logger::default())
            .configure(web::api::register)
            .service(afs::Files::new("/static", "static"))
            .route("/", get().to(LandingPage::handle))
            .route("/projects", get().to(ProjectsPage::handle))
            .route("/developers", get().to(DevelopersPage::handle))
            .route("/sponsors", get().to(SponsorsPage::handle))
            .route("/blog", get().to(blog::blog_service))
            .route("/login", post().to(login::login_service))
            .route("/logout", get().to(logout::logout_service))
            .route("/forgot", get().to(forgot::recovery_service))
            .route("/register", post().to(register::registration_service))
            .default_service(aweb::route().to(p404::not_found))
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
