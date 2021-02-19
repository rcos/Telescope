//! API interactions and functionality.

use crate::env::global_config;

pub mod auth;
pub mod introspect;
pub mod projects;
pub mod users;
pub mod models;

/// Get the URL that the central RCOS API is running at from the global config.
pub fn api_endpoint() -> String {
    global_config().api_url.clone()
}
