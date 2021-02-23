//! API interactions and functionality.

use crate::env::global_config;
use actix_web::client::Client;
use crate::error::TelescopeError;
use serde_json::Value;

mod auth;
pub mod projects;
pub mod users;
pub mod hasura;

/// Re-export client authenticated client constructor.
pub use auth::make_api_client;

/// The max size of the API response body in bytes. Any responses larger than this
/// will error on deconstruction.
const RESPONSE_BODY_LIMIT: usize = 1024*1024;

/// Get the URL that the central RCOS API is running at from the global config.
fn api_endpoint() -> String {
    global_config().api_url.clone()
}

// /// Send a GraphQL query to the central RCOS API using a given client.
// pub async fn send_graphql_query(client: Client, query: impl Into<String>) -> Result<Value, TelescopeError> {
//     // Create GraphQL request.
//     let request = gql::Request::new(query);
//
//     // Send the request and return the response.
//     return client
//         // POST request to the API endpoint.
//         .post(api_endpoint())
//         // The serialized JSON of the GraphQL request
//         .send_json(&request)
//         // Wait for the response
//         .await
//         // Convert and propagate any errors.
//         .map_err(TelescopeError::api_query_error)?
//         // The body of the response should be deserialized as JSON.
//         .json::<gql::Response>()
//         // Error if the body is larger than the limit.
//         .limit(RESPONSE_BODY_LIMIT)
//         // Wait for the body to deconstruct.
//         .await
//         // Convert and propagate any errors
//         .map_err(TelescopeError::api_response_error)?
//         // Convert the GraphQL response into a result type.
//         .into_result()
//         // Convert any GraphQL errors.
//         .map_err(|err| {
//             // Report all errors
//             err.iter()
//                 .for_each(|e| error!("GraphQL API returned an error: {}", e));
//             // Wrap in a telescope error
//             TelescopeError::GraphQLError(err)
//         })
//         // Convert the response to a JSON value.
//         .map(|ok_response| {
//             serde_json::to_value(ok_response.data)
//                 .expect("Could not convert GraphQL response to JSON value")
//         });
// }
