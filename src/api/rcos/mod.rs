//! API interactions and functionality.

use crate::api::handle_graphql_response;
use crate::api::rcos::auth::ApiJwtClaims;
use crate::env::global_config;
use crate::error::TelescopeError;
use graphql_client::{GraphQLQuery, Response as GraphQlResponse, QueryBody};
use reqwest::{header::HeaderValue, header::ACCEPT, Client};
use serde_json::Value;

mod auth;
pub mod landing_page_stats;
pub mod meetings;
pub mod prelude;
pub mod projects;
pub mod search_strings;
pub mod semesters;
pub mod users;

/// The name of this API in error messages.
const API_NAME: &'static str = "RCOS Central Hasura GraphQL API";

/// Send a GraphQL query to the central RCOS API for a given subject (or anonymously).
pub async fn send_query<T: GraphQLQuery>(
    variables: T::Variables,
) -> Result<T::ResponseData, TelescopeError> {
    // Build the GraphQL query.
    let query = T::build_query(variables);
    // Destructure the fields of the query.
    let QueryBody { operation_name, query, variables } = query;
    // Serialize the query variables to a JSON object.
    let variables: Value = serde_json::to_value(variables)
        .map_err(|e| TelescopeError::ise(format!("Could not serialize GraphQL variables to JSON object: {}", e)))?;

    // Build a JWT token to authenticate with the RCOS API.
    // Use no subject because currently we do not track the subject on
    // the other end.
    let jwt: String = ApiJwtClaims::new(None);

    // Send the query and await the response.
    let response: Value = send_json_query(operation_name, query, variables)
        .await?;

    // Deserialize the response into the typed value and return.
    serde_json::from_value::<T::ResponseData>(response)
        .map_err(|e| TelescopeError::ise(format!("Could not deserialize GraphQL API response: {}", e)))
}


/// Send an API query using the GraphQL JSON format. This is useful for avoiding issues in the
/// macro-generated GraphQL types
pub async fn send_json_query(query_name: &str, query_document: &str, variables: Value) -> Result<Value, TelescopeError> {
    // Build the GraphQL request body.
    let request_body: Value = json!({
        "query": query_document,
        "operationName": query_name,
        "variables": variables
    });

    // Build a JWT token to authenticate with the RCOS API.
    // Use no subject because currently we do not track the subject on
    // the other end.
    let jwt: String = ApiJwtClaims::new(None);

    // Create a new reqwest client
    return Client::new()
        // Create a POST request to the API endpoint.
        .post(global_config().api_url.as_str())
        // With the serialized JSON of the GraphQL request
        .json(&request_body)
        // And the JWT for authentication
        .bearer_auth(jwt)
        // Add the Accept header so that the server sends back JSON.
        .header(ACCEPT, HeaderValue::from_static("application/json"))
        // Send the request and wait for the response
        .send().await
        // Convert and propagate any errors.
        .map_err(TelescopeError::rcos_api_error)?
        // Wait for the body to receive as a string
        .text()
        .await
        // Convert and propagate any errors on deserializing the response body.
        .map_err(TelescopeError::rcos_api_error)
        // Convert the body into the GraphQL response type.
        .and_then(|body| {
            serde_json::from_str::<GraphQlResponse<Value>>(body.as_str())
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
        .and_then(|response| handle_graphql_response(API_NAME, response));
}
