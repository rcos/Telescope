//! Public meetings query.

use crate::web::api::rcos::prelude::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/public.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct PublicMeetings;
