//! GraphQL query to get user info to populate the embed for the `/whois` command on the
//! RCOS Discord bot.

use crate::web::api::rcos::prelude::*;

/// ZST representing the associated GraphQL query.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/discord_whois.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct DiscordWhoIs;

