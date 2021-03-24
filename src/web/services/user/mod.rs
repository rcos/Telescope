//! Services related to users.

use actix_web::web::ServiceConfig;

pub(crate) mod developers;
mod login;
pub(crate) mod profile;
mod register;

/// Register user related services.
pub fn register(config: &mut ServiceConfig) {
    config
        // User profile and settings
        .service(profile::profile)
        // Developers page
        .service(developers::developers_page)
        // Login related services.
        .service(login::login_page)
        .service(login::logout)
        // Registration related services
        .service(register::register_page)
        .service(register::finish_registration)
        .service(register::submit_registration);
}
