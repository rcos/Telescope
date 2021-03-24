//! Web services.

use actix_web::web::ServiceConfig;

pub mod auth;
mod index;
pub mod not_found;
mod projects;
pub(crate) mod user;
mod calendar;

/// Register all of the routs to the actix app.
pub fn register(config: &mut ServiceConfig) {
    // Register authentication related services
    auth::register(config);
    // Register user related services
    user::register(config);
    // Calendar related services.
    calendar::register(config);

    config
        // Homepage
        .service(index::index)
        // Projects page
        .service(projects::projects_page);
}
