//! API interactions and functionality.

use crate::env::global_config;
use graphql_client::{GraphQLQuery, Response as GraphQlResponse};
use actix_web::client::Client;
use crate::error::TelescopeError;

mod auth;
pub mod projects;
pub mod users;
pub mod hasura;
pub mod landing_page_stats;

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
pub async fn send_query<T: GraphQLQuery>(client: &Client, variables: T::Variables)
    -> Result<T::ResponseData, TelescopeError>
{
    // Build the GraphQL query
    let query = T::build_query(variables);

    // Send the request and return the response.
    return client
        // POST request to the API endpoint.
        .post(api_endpoint())
        // The serialized JSON of the GraphQL request
        .send_json(&query)
        // Wait for the response
        .await
        // Convert and propagate any errors.
        .map_err(TelescopeError::api_query_error)?
        // The body of the response should be deserialized as JSON.
        .json::<GraphQlResponse<T::ResponseData>>()
        // Error if the body is larger than the limit.
        .limit(RESPONSE_BODY_LIMIT)
        // Wait for the body to deconstruct.
        .await
        // Convert and propagate any errors on deserializing the response body.
        .map_err(TelescopeError::api_response_error)
        // Convert any GraphQL errors.
        .and_then(|response| match response {
            // If errors and data are both non-null
            GraphQlResponse {
                errors: Some(errs),
                data: Some(rdata)
            } => {
                if errs.is_empty() {
                    // If there are no errors return the data.
                    Ok(rdata)
                } else {
                    // If there are errors, return those.
                    Err(TelescopeError::GraphQLError(errs))
                }
            },

            // If no errors, return the data.
            GraphQlResponse {errors: None, data: Some(rdata)} => Ok(rdata),

            // If just errors, return those.
            GraphQlResponse {errors: Some(errs), data: None} => {
                if errs.is_empty() {
                    panic!("Central GraphQL API returned a response with no errors or data.");
                } else {
                    Err(TelescopeError::GraphQLError(errs))
                }
            }

            // Panic on None of either.
            GraphQlResponse {errors: None, data: None} =>
                panic!("Central GraphQL API responded with no errors or data.")
        });
}
