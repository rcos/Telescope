//! Services related to users.

use actix_web::web::ServiceConfig;

pub mod developers;
mod join_discord;
mod login;
pub mod profile;
mod register;
mod delete;

/// Register user related services.
pub fn register(config: &mut ServiceConfig) {
    // Developers page.
    developers::register_services(config);

    // User profile and settings.
    profile::register(config);

    // Everything else
    config
        // Login related services.
        .service(login::login_page)
        .service(login::logout)
        // Registration related services
        .service(register::register_page)
        .service(register::finish_registration)
        .service(register::submit_registration)
        // Discord Gateway
        .service(join_discord::handle)
        // User Deletion
        .service(delete::confirm_delete)
        .service(delete::profile_delete);
}
