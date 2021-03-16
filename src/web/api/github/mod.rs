//! GitHub API V4 queries and mutations.

use crate::error::TelescopeError;
use crate::web::api::handle_graphql_response;
use crate::web::telescope_ua;
use graphql_client::{GraphQLQuery, Response as GraphQLResponse};
use oauth2::AccessToken;
use reqwest::header::{HeaderValue, ACCEPT, USER_AGENT};
use reqwest::Client;

pub mod users;

/// The GitHub API endpoint
const GITHUB_API_ENDPOINT: &'static str = "https://api.github.com/graphql";

/// The name of this API in error reporting.
const API_NAME: &'static str = "GitHub API V4";

/// Send a GraphQL query to the GitHub API.
pub async fn send_query<T: GraphQLQuery>(
    auth_token: &AccessToken,
    variables: T::Variables,
) -> Result<T::ResponseData, TelescopeError> {
    // Build GraphQL request
    let query = T::build_query(variables);

    // Make a client, send the request, and return the result.
    return Client::new()
        // POST request to the GitHub GraphQL API endpoint
        .post(GITHUB_API_ENDPOINT)
        // With the JSON of the GraphQL query
        .json(&query)
        // With the user's access token
        .bearer_auth(auth_token.secret())
        // And required headers
        .header(ACCEPT, HeaderValue::from_static("application/json"))
        .header(USER_AGENT, telescope_ua())
        // Send and wait for a response
        .send()
        .await
        // Propagate any errors sending or receiving
        .map_err(TelescopeError::github_api_error)?
        // Get response as string
        .text()
        // Wait to receive the full response
        .await
        // Convert any errors.
        .map_err(TelescopeError::github_api_error)
        // Convert the valid JSON value into the GraphQL response type.
        .and_then(|body| {
            serde_json::from_str::<GraphQLResponse<T::ResponseData>>(body.as_str())
                // Convert serde error to telescope error
                .map_err(|err| {
                    // Log the error and response body
                    error!(
                        "Malformed GitHub API response: {}\nresponse body: {}",
                        err,
                        body.as_str()
                    );
                    // Convert error.
                    TelescopeError::GitHubApiError(err.to_string())
                })
        })
        // Convert any errors in the response
        .and_then(|response| handle_graphql_response::<T>(API_NAME, response));
}
