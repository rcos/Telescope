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

#[macro_use]
extern crate juniper;

pub mod util;

mod web;

mod db_janitor;
mod env;
mod models;
mod schema;
mod templates;

use crate::{
    db_janitor::DbJanitor,
    env::{ConcreteConfig, CONFIG},
    models::{emails::Email, password_requirements::PasswordRequirements, users::User},
    templates::static_pages::{
        index::LandingPage, projects::ProjectsPage, sponsors::SponsorsPage, Static,
    },
    web::{app_data::AppData, cookies, services, RequestContext},
};

//use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};

use actix::prelude::*;
use actix_files as afs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{http::Uri, middleware, web as aweb, web::get, App, HttpServer};
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use diesel::{Connection, RunQueryDsl};
use openssl::ssl::{SslAcceptor, SslMethod};
use rand::{rngs::OsRng, Rng};
use std::process::exit;

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

    if config
        .sysadmin_config
        .as_ref()
        .map(|c| c.create)
        .unwrap_or(false)
    {
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

        // I'm pretty sure this has to be written out manually in synchronous
        // diesel code here, since this is not an async context.
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
            // Redirect to https middleware
            .wrap(redirect_middleware.build())
            // logger middleware
            .wrap(middleware::Logger::default())
            // register API and Services
            .configure(web::api::register_apis)
            .configure(web::services::register)
            // static files service
            .service(afs::Files::new("/static", "static"))
            .route("/", get().to(Static::<LandingPage>::handle))
            .route("/projects", get().to(Static::<ProjectsPage>::handle))
            .route("/sponsors", get().to(Static::<SponsorsPage>::handle))
            .default_service(aweb::route().to(services::p404::not_found))
    })
    .bind(config.bind_http.clone())
    .expect("Could not bind HTTP/1 (HTTP)")
    .bind_openssl(config.bind_https.clone(), tls_builder)
    .expect("Could not bind HTTP/2 (HTTPS)")
    .run();

    // Start the actix runtime.
    sys.run()
}
