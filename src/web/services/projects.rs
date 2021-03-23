//! Projects related services

use crate::error::TelescopeError;
use actix_web::HttpResponse;

/// Projects page service
#[get("/projects")]
pub async fn projects_page() -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}
