//! Web services.

use actix_web::web::ServiceConfig;

mod admin;
pub mod auth;
mod coordinate;
mod index;
pub mod meetings;
pub mod not_found;
mod projects;
pub mod user;

/// Register all of the routes to the actix app.
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

    // Coordinator panel services.
    coordinate::register(config);

    config
        // Homepage
        .service(index::index);
}
