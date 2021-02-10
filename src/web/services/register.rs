use actix_web::HttpRequest;
use crate::templates::Template;
use crate::error::TelescopeError;

#[get("/register")]
/// Service for the registration page.
pub async fn register_page(req: HttpRequest) -> Result<Template, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}