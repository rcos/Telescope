use actix_web::web::ServiceConfig;

/// Rest API
pub mod rest;

pub fn register_apis(config: &mut ServiceConfig) {
    rest::register_api(config);
}
