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

#[macro_use]
extern crate graphql_client;

use actix::prelude::*;
use actix_files as afs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::cookie::SameSite;
use actix_web::{middleware, web as aweb, web::get, App, HttpServer};
use rand::rngs::OsRng;
use rand::Rng;

use crate::{
    templates::static_pages::{projects::ProjectsPage, sponsors::SponsorsPage, StaticPage},
    web::csrf::CsrfJanitor,
};

mod app_data;
mod env;
mod error;
mod templates;
mod web;

fn main() -> std::io::Result<()> {
    // set up logger and global web server configuration.
    env::init();

    // Create the actix runtime.
    let sys = System::new("telescope");

    // Start global CSRF token janitor.
    CsrfJanitor.start();

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
            // Cookie Identity middleware.
            .wrap(IdentityService::new(cookie_policy))
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
    .bind("0.0.0.0:80")
    .expect("Could not bind http://localhost:80")
    .run();

    // Start the actix runtime.
    sys.run()
}
