//! Functionality for inspecting the central RCOS API's schema.

use crate::error::TelescopeError;
use crate::env::{
    ConcreteConfig,
    global_config
};
use std::sync::Arc;
use actix_web::client::Client;
use actix_web::http::header::ACCEPT;
use serde_json::Value;

/// Query the central RCOS API for its schema and return it.
pub async fn unauthenticated_schema() -> Result<Value, TelescopeError> {
    // Get the global config in order to determine where the schema is hosted.
    let config: Arc<ConcreteConfig> = global_config();
    // Get the API URL from the config.
    let api_url: &str = config.api_url.as_str();

    // Create an HTTP client to send the request for the schema.
    let client: Client = Client::builder()
        // We should only accept JSON. If we don't get JSON from the
        // API endpoint then it is not in the OpenAPI Spec.
        .header(ACCEPT, "application/json")
        .finish();

    // Get and return the schema.
    return client
        // Create a GET request to the API URL.
        .get(api_url)
        // Send the request and wait for a response or an error.
        .send()
        .await
        // Convert and propagate any errors.
        .map_err(TelescopeError::api_query_error)?
        // The response should be a JSON serialization of an OpenAPI Spec.
        // Try to interpret it as one and propagte any errors that occur.
        .json::<Value>()
        .await
        .map_err(TelescopeError::api_response_error);
}

///