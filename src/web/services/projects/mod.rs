//! Services related to project management.

use actix_web::web::ServiceConfig;

mod projects_page;
mod view;

/// Register project services.
pub fn register(conf: &mut ServiceConfig) {
    conf.service(projects_page::get);
    conf.service(view::project);
}
