//! Query to get authenticated GitHub user.

// Import serializable URL type for query types.
use url::Url as URI;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/github/schema.json",
    query_path = "graphql/github/users/authenticated_user.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct AuthenticatedUser;
