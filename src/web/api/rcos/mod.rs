//! API interactions and functionality.

use crate::env::global_config;
use async_graphql as gql;
use actix_web::client::Client;
use crate::error::TelescopeError;

mod auth;
pub mod projects;
pub mod users;
pub mod models;

/// Re-export client authenticated client constructor.
pub use auth::make_api_client;

/// The max size of the API response body in bytes. Any responses larger than this
/// will error on deconstruction.
const RESPONSE_BODY_LIMIT: usize = 1024*1024;

/// Get the URL that the central RCOS API is running at from the global config.
fn api_endpoint() -> String {
    global_config().api_url.clone()
}

/// Send a GraphQL query to the central RCOS API using a given client.
pub async fn send_graphql_query(client: Client, query: impl Into<String>) -> Result<gql::Response, TelescopeError> {
    // Create GraphQL request.
    let request = gql::Request::new(query);

    // Send the request and return the response.
    return client
        // POST request to the API endpoint.
        .post(api_endpoint())
        // The serialized JSON of the GraphQL request
        .send_json(&request)
        // Wait for the response
        .await
        // Convert and propagate any errors.
        .map_err(TelescopeError::api_query_error)?
        // The body of the response should be deserialized as JSON.
        .json::<gql::Response>()
        // Error if the body is larger than the limit.
        .limit(RESPONSE_BODY_LIMIT)
        // Wait for the body to deconstruct.
        .await
        // Convert any errors.
        .map_err(TelescopeError::api_response_error);
}
