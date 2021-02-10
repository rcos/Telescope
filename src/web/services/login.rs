use actix_web::HttpRequest;
use crate::error::TelescopeError;
use crate::templates::{
    login,
    page,
    Template
};

#[get("/login")]
/// Login page. Users go here and are presented options to login with a variety
/// of identity providers.
pub async fn login_page(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Make the login page template.
    let content: Template = login::new();
    // Put it in a page template and return it.
    return page::of(req.path(), "RCOS Login", &content);
}
