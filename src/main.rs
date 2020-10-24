#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate diesel;

mod env;

mod web;
use web::*;

mod templates;

mod schema;

mod models;

use crate::{
    env::{
        Config,
        CONFIG
    },
    models::{Email, PasswordRequirements, User},
    templates::static_pages::{
        developers::DevelopersPage, index::LandingPage, projects::ProjectsPage,
        sponsors::SponsorsPage, Static,
    },
    web::app_data::AppData,
};

//use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};

use actix_files as afs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web as aweb, web::get, App, HttpServer};
use diesel::{Connection, RunQueryDsl};
use rand::{rngs::OsRng, Rng};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::process::exit;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set up logger and global web server configuration.
    env::init();
    let config: &Config = &*CONFIG;

    // Create appdata object.
    //
    // Database pool creation and registration of handlebars templates occurs
    // here.
    let app_data = AppData::new(config);

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
        .set_private_key_file(&config.tls_priv_key_file, SslFiletype::PEM)
        .map_err(|e| {
            error!(
                "Could not read TLS/SSL private key at {}: {}",
                config.tls_priv_key_file, e
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

    if config.create_sysadmin {
        let admin_password = config.sysadmin_password.as_ref().unwrap();
        let admin_email = config.sysadmin_email.as_ref().unwrap();
        let mut user: User = User::new("Telescope admin", admin_password)
            .map_err(|e: PasswordRequirements| {
                error!(
                    "Admin password {} failed to satisfy password requirements.",
                    admin_password
                );
                if !e.not_common_password {
                    error!(
                        "Admin password {} is too common. Please choose a different password.",
                        admin_password
                    );
                }
                if !e.is_min_len {
                    error!(
                        "Admin password {} is too short. \
                        Please choose a password more than {} characters.",
                        admin_password,
                        PasswordRequirements::MIN_PASSWORD_LENGTH
                    )
                }
                exit(exitcode::DATAERR)
            })
            .unwrap();
        let email = Email::new(user.id, admin_email).unwrap_or_else(|| {
            error!("Admin email {} is not a valid email.", admin_email);
            exit(exitcode::DATAERR);
        });

        user.sysadmin = true;

        let pool = app_data.clone_db_conn_pool();
        let conn = pool.get().unwrap();
        conn.transaction::<(), diesel::result::Error, _>(|| {
            use crate::schema::emails::dsl::emails;
            use crate::schema::users::dsl::users;

            diesel::insert_into(users).values(&user).execute(&conn)?;

            diesel::insert_into(emails).values(email).execute(&conn)?;

            info!("Successfully added admin user (id: {})", user.id_str());
            Ok(())
        })
        .map_err(|e| {
            error!("Could not add admin user to database: {}", e);
            e
        })
        .unwrap();
    }

    // generate a random key to encrypt cookies.
    let cookie_key = OsRng::default().gen::<[u8; 32]>();

    /*
    // memory store for rate limiting.
    let ratelimit_memstore = MemoryStore::new();
     */

    HttpServer::new(move || {
        App::new()
            .data(app_data.clone())
            // Identity and authentication middleware.
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookie_key)
                    .name(cookies::AUTH_TOKEN)
                    .secure(true)
                    // Cookies / sessions expire after 24 hours
                    .max_age_time(time::Duration::hours(24)),
            ))
            /*
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(ratelimit_memstore.clone()).start())
                    // rate limit: 100 requests max per minute
                    .with_interval(std::time::Duration::from_secs(60))
                    .with_max_requests(100),
            )
             */
            // logger middleware
            .wrap(middleware::Logger::default())
            // register API and Services
            .configure(web::api::register_apis)
            .configure(web::services::register)
            // static files service
            .service(afs::Files::new("/static", "static"))
            .route("/", get().to(Static::<LandingPage>::handle))
            .route("/projects", get().to(Static::<ProjectsPage>::handle))
            .route("/developers", get().to(Static::<DevelopersPage>::handle))
            .route("/sponsors", get().to(Static::<SponsorsPage>::handle))
            /*
            .route("/blog", get().to(blog::blog_service))
            .route("/logout", get().to(logout::logout_service))
            .route("/forgot", get().to(forgot::recovery_service))
            .route("/register", post().to(register::registration_service))
            */
            .default_service(aweb::route().to(services::p404::not_found))
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
