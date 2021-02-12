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

/// Service that serves the telescope homepage.
#[get("/")]
pub async fn index(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Get central API URL.
    let config: Arc<ConcreteConfig> = global_config();
    let api_url: &str = config.api_url.as_str();
    // Fetch the project list from API.
    // Create a client.
    let client: Client = Client::builder()
        // We only want JSON formatted results from the API.
        .header(ACCEPT, "application/json")
        .finish();

    // Assume that the URL ends with a backslash.
    let projects = client.get(format!("{}projects", api_url))
        // Send the request.
        .send()
        // Wait for the response.
        .await
        // Convert and propagate any errors.
        .map_err(TelescopeError::api_query_error)?
        // The body should be JSON. Try to convert it.
        .json::<Value>()
        .await
        // Again, convert and propagate any errors.
        .map_err(TelescopeError::api_response_error)?;

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
