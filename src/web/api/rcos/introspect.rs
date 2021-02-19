//! Functionality for inspecting the central RCOS API's schema.

use crate::error::TelescopeError;
use actix_web::client::Client;
use serde_json::Value;
use super::auth::*;
use super::api_endpoint;

/// Function to get a schema using a given client.
async fn schema(client: Client) -> Result<Value, TelescopeError> {
    client
        // Create a GET request to the API URL.
        .get(api_endpoint())
        // Send the request and wait for a response or an error.
        .send()
        .await
        // Convert and propagate any errors.
        .map_err(TelescopeError::api_query_error)?
        // The response should be a JSON serialization of an OpenAPI Spec.
        // Try to interpret it as one and propagte any errors that occur.
        .json::<Value>()
        // We have to change this limit here just due to the sheer size of the
        // schema returned. This limit may also have to grow as the schema
        // changes.
        .limit(1024*1024)
        .await
        .map_err(TelescopeError::api_response_error)
}

/// Query the central RCOS API for its schema and return it.
pub async fn unauthenticated_schema() -> Result<Value, TelescopeError> {
    return schema(make_api_client(ANONYMOUS_USER, ACCEPT_JSON)).await;
}

/// Query the central RCOS API for its schema as an authenticated user.
/// This just adds a JWT with the "role" claim to the request.
pub async fn authenticated_schema() -> Result<Value, TelescopeError> {
    return schema(make_api_client(AUTHENTICATED_USER, ACCEPT_JSON)).await;
}
