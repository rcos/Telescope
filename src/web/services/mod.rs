//! Web services.

use actix_web::web::ServiceConfig;

mod admin;
pub mod auth;
mod index;
pub mod meetings;
pub mod not_found;
mod projects;
pub mod user;

/// Register all of the routs to the actix app.
pub fn register(config: &mut ServiceConfig) {
    // Register authentication related services
    auth::register(config);

    // Register user related services
    user::register(config);

    // Calendar related services.
    meetings::register(config);

    // Project related services.
    projects::register(config);

    // Admin panel services.
    admin::register(config);

    config
        // Homepage
        .service(index::index);
}
