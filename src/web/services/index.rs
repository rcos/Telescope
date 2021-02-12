//! Module for serving the RCOS homepage.

use actix_web::HttpRequest;
use crate::templates::{
    Template,
    homepage
};
use crate::error::TelescopeError;
use crate::env::global_config;

/// Service that serves the telescope homepage.
#[get("/")]
pub async fn index(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Get central API URL.
    let api_url: &str = global_config().api_url.as_str();
    

    let template: Template = homepage::new(
        "(Semester Unknown)",
        "API Unimplemented",
        "API Unimplemented",
        "API Unimplemented",
        "API Unimplemented",
    );
}
