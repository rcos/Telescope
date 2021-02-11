use crate::error::TelescopeError;
use actix_web::{HttpRequest, HttpResponse};

#[get("/login/rpi_cas")]
pub async fn login(req: HttpRequest) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}
