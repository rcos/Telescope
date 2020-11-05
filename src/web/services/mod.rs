use actix_web::web::ServiceConfig;

pub mod blog;
pub mod forgot;
pub mod login;
pub mod logout;
pub mod p404;
pub mod profile;
pub mod register;
pub mod confirm;

/// Register services to the actix-web server config.
pub fn register(config: &mut ServiceConfig) {
    config
        .service(login::login_get)
        .service(login::login_post)
        .service(logout::logout_service)
        .service(forgot::forgot_page)
        .service(forgot::recovery_service)
        .service(register::registration_service)
        .service(confirm::confirmations_page)
        .service(register::signup_page)
        .service(profile::profile);
}
