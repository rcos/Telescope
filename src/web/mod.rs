//! Web services and utilities.

use reqwest::header::HeaderValue;

pub mod api;
pub mod csrf;
pub mod error_rendering_middleware;
pub mod services;

lazy_static! {
    static ref TELESCOPE_USER_AGENT: String =
        format!("rcos-telescope/{}", env!("CARGO_PKG_VERSION"));
}

/// Get the telescope User-Agent string.
pub fn telescope_ua() -> HeaderValue {
    HeaderValue::from_str(TELESCOPE_USER_AGENT.as_str())
        .expect("Could not make Telescope User-Agent")
}
