//! API interactions and functionality.

use actix_web::client::Client;
use crate::env::global_config;
use serde_json::Value;
use crate::error::TelescopeError;

mod introspect;
mod projects;
pub mod auth;

/// Get the URL that the central RCOS API is running at from the global config.
pub fn api_endpoint() -> String {
    global_config().api_url.clone()
}

/// Send a request as an unauthenticated user.
pub async fn send_unauthenticated() -> Result<Value, TelescopeError> {
    unimplemented!()
}
