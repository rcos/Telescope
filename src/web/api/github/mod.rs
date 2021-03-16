//! GitHub API V4 queries and mutations.

use graphql_client::{GraphQLQuery, Response as GraphQLResponse};
use oauth2::AccessToken;
use crate::error::TelescopeError;
use reqwest::Client;
use crate::web::api::handle_graphql_response;
use serde_json::Value;

pub mod users;

/// The GitHub API endpoint
const GITHUB_API_ENDPOINT: &'static str = "https://api.github.com/graphql";

/// The name of this API in error reporting.
const API_NAME: &'static str = "GitHub API V4";

/// Send a GraphQL query to the GitHub API.
pub async fn send_query<T: GraphQLQuery>(auth_token: &AccessToken, variables: T::Variables) -> Result<T::ResponseData, TelescopeError> {
    // Build GraphQL request
    let request = T::build_query(variables);

    // Make a client, send the request, and return the result.
    return Client::new()
        // POST request to the GitHub GraphQL API endpoint
        .post(GITHUB_API_ENDPOINT)
        // With the user's access token
        .bearer_auth(auth_token.secret())
        // Send and wait for a response
        .send()
        .await
        // Propagate any errors sending or receiving
        .map_err(TelescopeError::github_api_error)?
        // Deserialize response as JSON
        .json::<Value>()
        // Wait for the full response to deserialize
        .await
        // Convert any errors.
        .map_err(TelescopeError::github_api_error)
        // Convert the valid JSON value into the GraphQL response type.
        .and_then(|json_value| serde_json::from_value::<GraphQLResponse<T::ResponseData>>(json_value.clone())
            // Convert serde error to telescope error
            .map_err(|err| {
                // Log the error and response body
                error!("Malformed GitHub API response: {}\nresponse body: {}",
                       err,
                       serde_json::to_string_pretty(&json_value).expect("Could not display response body"));
                // Convert error.
                TelescopeError::GitHubApiError(err.to_string())
            }))
        // Convert any errors in the response
        .and_then(|response| handle_graphql_response::<T>(API_NAME, response))
}
