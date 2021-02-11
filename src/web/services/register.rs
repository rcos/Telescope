use crate::error::TelescopeError;
use crate::templates::{auth, page, Template};
use actix_web::HttpRequest;

#[get("/register")]
/// Service for the registration page. This page allows users to start the
/// account creation process by signing into an identity provider.
pub async fn register_page(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Make the create account page template.
    let content: Template = auth::register();
    // Put it in a page template and return it.
    return page::of(req.path(), "RCOS Login", &content);
}
