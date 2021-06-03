//! Services for the semester records management page.

use actix_web::web::ServiceConfig;
use actix_web::HttpRequest;
use crate::web::services::auth::identity::AuthenticationCookie;
use crate::templates::Template;
use crate::error::TelescopeError;

/// Register semester services.
pub fn register(config: &mut ServiceConfig) {
    config.service(index);
}

#[get("/semesters")]
async fn index(req: HttpRequest, auth: AuthenticationCookie) -> Result<Template, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}
