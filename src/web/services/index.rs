//! Module for serving the RCOS homepage.

use actix_web::HttpRequest;
use crate::templates::{
    Template,
    homepage,
    page
};
use crate::error::TelescopeError;
use crate::env::{global_config, ConcreteConfig};
use actix_web::client::Client;
use actix_web::http::header::ACCEPT;
use crate::models::projects::Project;
use std::sync::Arc;
use serde_json::Value;
use crate::web::api;

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
