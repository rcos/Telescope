use actix_web::{HttpResponse, HttpRequest};
use crate::error::TelescopeError;

#[get("/login/rpi_cas")]
pub async fn login(req: HttpRequest) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}