//! Web services.

pub mod auth;
mod login;
pub mod not_found;
mod register;

use actix_web::web::ServiceConfig;

/// Register all of the routs to the actix app.
pub fn register(config: &mut ServiceConfig) {
    // Register authentication related services
    auth::register(config);

    config
        // Login services.
        .service(login::login_page)
        // Account registration services.
        .service(register::register_page);
}
