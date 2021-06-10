//! Project page.

use crate::error::TelescopeError;
use actix_web::HttpResponse;

#[get("/projects")]
pub async fn get() -> Result<HttpResponse, TelescopeError> {
    return Err(TelescopeError::NotImplemented);
}
