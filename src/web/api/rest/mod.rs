use actix_web::web::ServiceConfig;

pub mod login;

/// Register the rest api
pub fn register_api(config: &mut ServiceConfig) {
    config
        .service(login::login_rest);
}