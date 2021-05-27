//! Project page.

use actix_web::HttpResponse;
use crate::error::TelescopeError;

#[get("/projects")]
pub async fn get() -> Result<HttpResponse, TelescopeError> {
    return Err(TelescopeError::NotImplemented);
}
