//! Web services and utilities.

use reqwest::header::HeaderValue;
use crate::web::services::user::profile::ProfileQuery;

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

/// Get the profile path for a given username.
pub fn profile_for(username: &str) -> String {
    // Encode the username
    let encoded: String = serde_urlencoded::to_string(ProfileQuery {
        username: username.to_string()
    }).expect("Could not URL-encode username");

    // Put it in the correct part of the query for now.
    return format!("/user?{}", encoded);
}
