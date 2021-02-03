use crate::error::TelescopeError;
use actix_web::HttpResponse;

// Use HttpResponse here because never type is not yet stable.
/// Respond to all requests with page not found.
/// Used as default service.
pub async fn not_found() -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::PageNotFound)
}
