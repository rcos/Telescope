use actix_web::web::ServiceConfig;

/// Rest API
pub mod rest;

/// GraphQL API
pub mod graphql;

pub fn register_apis(config: &mut ServiceConfig) {
    graphql::register(config);
    rest::register_api(config);
}