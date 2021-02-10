//! Web services.

pub mod not_found;
mod login;
mod auth;

use actix_web::web::ServiceConfig;

/// Register all of the routs to the actix app.
pub fn register(config: &mut ServiceConfig) {
    config
        // Login & authentication related services.
        .service(login::login_page);
}
