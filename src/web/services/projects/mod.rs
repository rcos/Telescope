//! Services related to project management.
use crate::error::TelescopeError;
use actix_web::web::ServiceConfig;

mod projects_page;

/// Register project services.
pub fn register(config: &mut ServiceConfig) {
    config.service(projects_page::projects_list);
}
