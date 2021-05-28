//! GraphQL query to get a paginated list of RCOS projects.

use crate::api::rcos::prelude::*;

/// GraphQL query to get projects with enrollments in an ongoing semester.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/projects.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct CurrentProjects;

/// GraphQL query to get all projects.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/projects.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct AllProjects;
