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

mod web;
use web::*;

mod env;
mod templates;
mod schema;
mod models;
mod db_janitor;

use crate::{
    env::{
        ConcreteConfig,
        CONFIG
    },
    models::{Email, PasswordRequirements, User},
    templates::static_pages::{
        developers::DevelopersPage, index::LandingPage, projects::ProjectsPage,
        sponsors::SponsorsPage, Static,
    },
    web::app_data::AppData,
    db_janitor::DbJanitor,
};

//use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};

use actix_files as afs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web as aweb, web::get, App, HttpServer};
use diesel::{Connection, RunQueryDsl};
use rand::{rngs::OsRng, Rng};
use openssl::ssl::{SslAcceptor, SslMethod};
use std::process::exit;
use actix::prelude::*;

fn main() -> std::io::Result<()> {
    // set up logger and global web server configuration.
    env::init();
    let config: &ConcreteConfig = &*CONFIG;

    // Create the actix runtime.
    let sys = System::new("telescope");

    // Create appdata object.
    //
    // Database pool creation and registration of handlebars templates occurs
    // here.
    let app_data = AppData::new();

    // from example at https://actix.rs/docs/http2/
    // to generate a self-signed certificate and private key for testing, use
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout tls-ssl/private-key.pem -out tls-ssl/certificate.pem -days 365`
    let mut tls_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Could not create SSL Acceptor.");
    config.tls_config.init_tls_acceptor(&mut tls_builder);

    if config.sysadmin_config.as_ref().map(|c| c.create).unwrap_or(false) {
        let config = config.sysadmin_config.clone().unwrap();
        let admin_password = config.password.as_str();
        let admin_email = config.email;
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
        let email = Email::new_prechecked(user.id, admin_email);

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

    // Database janitor -- This actor runs a few database operations once every
    // 24 hours to clear out expired database records.
    let db_pool = app_data.clone_db_conn_pool();
    DbJanitor::new(db_pool).start();

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
            .default_service(aweb::route().to(services::p404::not_found))
    })
        .bind_openssl(config.bind_to.clone(), tls_builder)
        .map_err(|e| {
            error!("Could not bind to {}: {}", config.bind_to, e);
            e
        })?
        .run();

    // Start the actix runtime.
    sys.run()
}
