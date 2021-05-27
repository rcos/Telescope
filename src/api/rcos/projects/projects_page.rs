//! GraphQL query to get a paginated list of RCOS projects.

use crate::api::rcos::prelude::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/projects.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Projects;

