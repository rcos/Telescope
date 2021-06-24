//! Meetings page and services

use actix_web::web::ServiceConfig;

mod create;
mod delete;
mod edit;
mod list;
mod view;

/// Register calendar related services.
pub fn register(config: &mut ServiceConfig) {
    // Meetings list page
    list::register(config);

    // Meeting creation services
    create::register(config);

    // Meeting edit services.
    edit::register(config);

    // Meeting destruction services.
    delete::register(config);

    config
        // The meeting viewing endpoint must be registered after the meeting creation endpoint,
        // so that the ID path doesn't match the create path.
        .service(view::meeting);
}
