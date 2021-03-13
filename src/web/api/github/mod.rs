//! GitHub API V4 queries and mutations.

use graphql_client::{GraphQLQuery, Response as GraphQLResponse};
use oauth2::AccessToken;
use crate::error::TelescopeError;
use reqwest::Client;
use crate::web::api::handle_graphql_response;

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
        .json::<GraphQLResponse<T::ResponseData>>()
        // Wait for the full response to deserialize
        .await
        // Convert any errors.
        .map_err(TelescopeError::github_api_error)
        // Convert any errors in the response
        .and_then(|response| handle_graphql_response::<T>(API_NAME, response))
}
