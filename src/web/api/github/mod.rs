//! GitHub API V4 queries and mutations.

use graphql_client::GraphQLQuery;
use oauth2::AccessToken;
use crate::error::TelescopeError;

pub mod users;

/// The GitHub API endpoint
const GITHUB_API_ENDPOINT: &'static str = "https://api.github.com/graphql";

/// Send a GraphQL query to the GitHub API.
pub async fn send_query<T: GraphQLQuery>(auth_token: AccessToken, variables: T::Variables) -> Result<T::ResponseData, TelescopeError> {
    unimplemented!()
}
