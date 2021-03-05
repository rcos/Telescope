//! Login and logout

use crate::error::TelescopeError;
use crate::templates::{auth, page, Template};
use crate::web::services::auth::identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{HttpRequest, HttpResponse};

#[get("/login")]
/// Login page. Users go here and are presented options to login with a variety
/// of identity providers.
pub async fn login_page(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Make the login page template.
    let content: Template = auth::login();
    // Put it in a page template and return it.
    return page::of(&req, "RCOS Login", &content).await;
}

#[get("/logout")]
/// Logout service. This just logs the user out and then redirects them to the
/// homepage.
pub async fn logout(identity: Identity) -> HttpResponse {
    // Forget the user's identity
    identity.forget();
    // Redirect the user to the homepage.
    HttpResponse::Found().header(LOCATION, "/").finish()
}
