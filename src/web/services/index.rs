//! Module for serving the RCOS homepage.

use actix_web::HttpRequest;
use crate::templates::{
    Template,
    homepage,
    page
};
use crate::error::TelescopeError;

/// Service that serves the telescope homepage.
#[get("/")]
pub async fn index(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Get the API schema
    // let schema: Value = api::unauthenticated_schema().await?;

    // Make the homepage template as the content of the landing page.
    let content: Template = homepage::new(
        "(Semester Unknown)",
        "API Unimplemented",
        "API Unimplemented",
        "API Unimplemented",
        "API Unimplemented",
    );

    // Return a page with the homepage content.
    return page::of(req.path(), "RCOS", &content);
}
