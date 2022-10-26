//! Services related to project management.

use actix_web::web::ServiceConfig;

mod list;
mod view;

/// Register project services.
pub fn register(conf: &mut ServiceConfig) {
    conf.service(list::get);
    conf.service(view::project);
}
