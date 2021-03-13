//! API interactions and functionality.

use crate::env::global_config;
use crate::error::TelescopeError;
use crate::web::api::rcos::auth::ApiJwtClaims;
use graphql_client::{GraphQLQuery, Response as GraphQlResponse};
use reqwest::{header::HeaderValue, header::ACCEPT, Client};
use crate::web::api::handle_graphql_response;

mod auth;
pub mod landing_page_stats;
pub mod users;

/// The name of this API in error messages.
const API_NAME: &'static str = "RCOS Central Hasura GraphQL API";

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
        .and_then(|response| handle_graphql_response::<T>(API_NAME, response));
}
