//! Web services.

pub mod auth;
pub mod developers;
mod index;
mod login;
pub mod not_found;
mod register;

use actix_web::web::ServiceConfig;

/// Register all of the routs to the actix app.
pub fn register(config: &mut ServiceConfig) {
    // Register authentication related services
    auth::register(config);

    config
        // Homepage.
        .service(index::index)
        // Login services.
        .service(login::login_page)
        .service(login::logout)
        // Account registration services.
        .service(register::register_page)
        .service(register::finish_registration)
        .service(register::submit_registration)
        // Developers Page
        .service(developers::developers_page);
}
