//! Projects related services

use actix_web::HttpResponse;
use crate::error::TelescopeError;

/// Projects page service
#[get("/projects")]
pub async fn projects_page() -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}
