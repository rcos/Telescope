//! API interactions and functionality.

use crate::env::global_config;
use crate::error::TelescopeError;
use crate::web::api::rcos::auth::ApiJwtClaims;
use graphql_client::{GraphQLQuery, Response as GraphQlResponse};
use reqwest::{header::HeaderValue, header::ACCEPT, Client};

mod auth;
pub mod landing_page_stats;
pub mod users;

/// Send a GraphQL query to the central RCOS API for a given subject (or anonymously).
pub async fn send_query<T: GraphQLQuery>(
    subject: Option<String>,
    variables: T::Variables,
) -> Result<T::ResponseData, TelescopeError> {
    // Build the GraphQL query
    let query = T::build_query(variables);

    // Build a JWT token to authenticate with the RCOS API.
    let jwt: String = ApiJwtClaims::new(subject);

    // Create a new reqwest client
    return Client::new()
        // Create a POST request to the API endpoint.
        .post(global_config().api_url.as_str())
        // With the serialized JSON of the GraphQL request
        .json(&query)
        // And the JWT for authentication
        .bearer_auth(jwt)
        // Add the Accept header so that the server sends back JSON.
        .header(ACCEPT, HeaderValue::from_static("application/json"))
        // Send the request and wait for the response
        .send()
        .await
        // Convert and propagate any errors.
        .map_err(TelescopeError::rcos_api_error)?
        // The body of the response should be deserialized as JSON.
        .json::<GraphQlResponse<T::ResponseData>>()
        // Wait for the body to deconstruct.
        .await
        // Convert and propagate any errors on deserializing the response body.
        .map_err(TelescopeError::rcos_api_error)
        // Convert any GraphQL errors.
        .and_then(|response| match response {
            // If errors and data are both non-null
            GraphQlResponse {
                errors: Some(errs),
                data: Some(rdata),
            } => {
                if errs.is_empty() {
                    // If there are no errors return the data.
                    Ok(rdata)
                } else {
                    // If there are errors, return those.
                    Err(TelescopeError::GraphQLError(errs))
                }
            }

            // If no errors, return the data.
            GraphQlResponse {
                errors: None,
                data: Some(rdata),
            } => Ok(rdata),

            // If just errors, return those.
            GraphQlResponse {
                errors: Some(errs),
                data: None,
            } => {
                if errs.is_empty() {
                    panic!("Central GraphQL API returned a response with no errors or data.");
                } else {
                    Err(TelescopeError::GraphQLError(errs))
                }
            }

            // Panic on None of either.
            GraphQlResponse {
                errors: None,
                data: None,
            } => panic!("Central GraphQL API responded with no errors or data."),
        });
}
