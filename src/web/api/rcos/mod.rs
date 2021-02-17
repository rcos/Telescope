//! API interactions and functionality.

use crate::env::global_config;

// Re-export client generators.
mod auth;
pub use auth::*;

pub mod introspect;
pub mod projects;

/// Get the URL that the central RCOS API is running at from the global config.
pub fn api_endpoint() -> String {
    global_config().api_url.clone()
}

