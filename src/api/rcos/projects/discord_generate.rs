
use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// ZST representing the associated GraphQL query.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/discord_whois.graphql",
    response_derives = "Debug,Clone,Serialize"
)]