//! Services related to users.

use actix_web::web::ServiceConfig;

mod login;
mod profile;
mod register;
pub(crate) mod developers;

/// Register user related services.
pub fn register(config: &mut ServiceConfig) {
    config
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
