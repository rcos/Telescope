#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate graphql_client;

use crate::discord_bot::DiscordBot;
use crate::templates::static_pages::sponsors::SponsorsPage;
use crate::templates::static_pages::StaticPage;
use crate::web::csrf::CsrfJanitor;
use crate::web::middlewares;
use actix::prelude::*;
use actix_files as afs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::cookie::SameSite;
use actix_web::{middleware, web as aweb, web::get, App, HttpServer};
use chrono::Offset;
use rand::rngs::OsRng;
use rand::Rng;

pub mod api;
mod app_data;
mod discord_bot;
mod env;
mod error;
mod templates;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set up logger and global web server configuration.
    env::init();
    // Log the server timezone
    info!("Server timezone: {}", chrono::Local::now().offset().fix());

    // Start global CSRF token janitor.
    CsrfJanitor.start();

    // Create and start the discord bot under a Supervisor that will
    // restart it if it crashes.
    Supervisor::start(|_| DiscordBot);

    // Setup identity middleware.
    // Create secure random sequence to encrypt cookie identities.
    let cookie_key: [u8; 32] = OsRng::default().gen::<[u8; 32]>();

    // Construct and start main server instance.
    let web_server = HttpServer::new(move || {
        // Create cookie policy.
        let cookie_policy = CookieIdentityPolicy::new(&cookie_key)
            // Transmit cookies over HTTPS only.
            .secure(true)
            .name("telescope_auth")
            // Same-Site needs to be Lax because of the caddy proxy it seems?
            .same_site(SameSite::Lax)
            // Cookies expire after a day.
            .max_age_time(time::Duration::days(1));

        App::new()
            // Middleware to render telescope errors into pages
            .wrap(middlewares::error_rendering::TelescopeErrorHandler)
            // Cookie Identity middleware.
            .wrap(IdentityService::new(cookie_policy))
            // Logger middleware
            .wrap(middleware::Logger::default())
            // Register Services
            .configure(web::services::register)
            // static files service
            .service(
                afs::Files::new("/static", "static")
                    // Text responses are UTF-8
                    .prefer_utf8(true)
                    // Show listings of directories
                    .show_files_listing(),
            )
            .route("/sponsors", get().to(SponsorsPage::page))
            .default_service(aweb::to(web::services::not_found::not_found))
    })
    // Bind to 80 (this gets reversed proxied by Caddy later)
    .bind("0.0.0.0:80")
    .expect("Could not bind http://localhost:80")
    // Start the server running.
    .run();

    // Wait on server to produce an error.
    return web_server.await;
}
