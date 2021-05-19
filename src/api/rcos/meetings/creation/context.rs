//! GraphQL query to get context for meeting creation.

use crate::api::rcos::prelude::*;

/// ZST representing the GraphQL query to resolve meeting creation context.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/creation/context.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct MeetingCreationContext;

