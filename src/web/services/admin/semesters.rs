//! Services for the semester records management page.

use actix_web::web::ServiceConfig;
use crate::templates::Template;
use crate::error::TelescopeError;

/// Register semester services.
pub fn register(config: &mut ServiceConfig) {
    config.service(index);
}

/// Page to display previous semesters and allow edits.
#[get("/semesters")]
async fn index() -> Result<Template, TelescopeError> {

    Err(TelescopeError::NotImplemented)
}
