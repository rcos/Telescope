//! GraphQL query to check if a user is a coordinator in any given semester.

use crate::web::api::rcos::prelude::*;

/// Type representing GraphQL query to get a user's coordinating status over several semesters.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/enrollments/is_coordinator.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct IsCoordinator;

