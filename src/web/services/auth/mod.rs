mod github;
mod rpi_cas;

use actix_web::web::ServiceConfig;

/// Register auth services.
pub fn register(config: &mut ServiceConfig) {
    config
        // GitHub related services.
        .service(github::login)
        // RPI CAS related services.
        .service(rpi_cas::login);
}