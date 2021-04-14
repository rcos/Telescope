//! GraphQL query for info about the current semester.

use crate::web::api::rcos::prelude::*;

/// Type representing GraphQL query for current semester data.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/semesters/current/info.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct CurrentSemesters;
