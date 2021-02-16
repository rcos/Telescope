//! API interactions and functionality.

use actix_web::client::Client;
use crate::env::global_config;
use serde_json::Value;
use crate::error::TelescopeError;

// Re-export client generators.
mod auth;
pub use auth::*;

mod introspect;
mod projects;

/// Get the URL that the central RCOS API is running at from the global config.
pub fn api_endpoint() -> String {
    global_config().api_url.clone()
}

