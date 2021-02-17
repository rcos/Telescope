//! Web services.

pub mod auth;
mod login;
pub mod not_found;
mod register;
mod index;

use actix_web::web::{ServiceConfig, Json};
use serde_json::Value;
use crate::error::TelescopeError;
use crate::web::api;
use actix_web::dev::Service;

/// Register all of the routs to the actix app.
pub fn register(config: &mut ServiceConfig) {
    // Register authentication related services
    auth::register(config);

    config
        // Homepage.
        .service(index::index)
        // Login services.
        .service(login::login_page)
        // Account registration services.
        .service(register::register_page)

        // Temporarily service the authenticated schema spec
        .service(get_authenticated_api);
}

/// Temporary service to inspect the authenticated schema.
#[get("/authenticated_schema")]
async fn get_authenticated_api() -> Result<Json<Value>, TelescopeError> {
    // Get the authenticated schema spec.
    return api::rcos::introspect::authenticated_schema()
        .await
        .map(|v| Json(v));
}