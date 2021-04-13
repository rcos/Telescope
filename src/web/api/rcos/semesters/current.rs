//! GraphQL query to get the current semester(s).

use crate::web::api::rcos::prelude::*;

/// Type representing GraphQL query for current semester data.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/semesters/current.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct CurrentSemesters;

