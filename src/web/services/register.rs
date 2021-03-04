use crate::error::TelescopeError;
use crate::templates::{auth, page, Template};
use actix_web::{HttpRequest, HttpResponse};
use crate::web::services::auth::identity::{Identity, IdentityCookie};
use crate::templates::forms::{
    Form,
    register
};

#[get("/register")]
/// Service for the registration page. This page allows users to start the
/// account creation process by signing into an identity provider.
pub async fn register_page(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Make the create account page template.
    let content: Template = auth::register();
    // Put it in a page template and return it.
    return page::of(req.path(), "Create RCOS Account", &content);
}

#[get("/register/finish")]
/// Users finish the registration process by supplying their first and last name. Telescope creates
/// the necessary records in the RCOS database via the central API. Argument extractors will error
/// if the identity is not authenticated.
pub async fn finish_registration(req: HttpRequest, identity_cookie: IdentityCookie) -> Result<Form, TelescopeError> {
    // Create a form for the authenticated the user's cookie.
    register::for_identity(identity_cookie).await
}

#[post("/register/finish")]
/// Endpoint to which users submit their forms. Argument extractor will error if user is not
/// authenticated.
pub async fn submit_registration(req: HttpRequest, cookie: IdentityCookie) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}