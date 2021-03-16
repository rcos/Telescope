//! API interactions and functionality.

use crate::env::global_config;
use crate::error::TelescopeError;
use crate::web::api::handle_graphql_response;
use crate::web::api::rcos::auth::ApiJwtClaims;
use graphql_client::{GraphQLQuery, Response as GraphQlResponse};
use reqwest::{header::HeaderValue, header::ACCEPT, Client};

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
        // Wait for the body to receive as a string
        .text()
        .await
        // Convert and propagate any errors on deserializing the response body.
        .map_err(TelescopeError::rcos_api_error)
        // Convert the body into the GraphQL response type.
        .and_then(|body| {
            serde_json::from_str::<GraphQlResponse<T::ResponseData>>(body.as_str())
                // Map Serde errors into telescope errors
                .map_err(|err| {
                    // Log the error and response body.
                    error!(
                        "Error querying RCOS API: {}\nresponse body: {}",
                        err,
                        body.as_str()
                    );
                    // Convert the error
                    TelescopeError::RcosApiError(err.to_string())
                })
        })
        // Convert any GraphQL errors.
        .and_then(|response| handle_graphql_response::<T>(API_NAME, response));
}
