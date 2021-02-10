use actix_web::{HttpRequest, HttpResponse};
use crate::error::TelescopeError;

#[get("login/github")]
pub async fn login(req: HttpRequest) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}