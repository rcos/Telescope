//! GraphQL query to get user info to populate the embed for the `/whois` command on the
//! RCOS Discord bot.

use crate::api::rcos::prelude::*;

/// ZST representing the associated GraphQL query.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/discord_whois.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct DiscordWhoIs;

use discord_who_is::Variables;

pub use discord_who_is::ResponseData;
use crate::error::TelescopeError;
use crate::api::rcos::send_query;
use chrono::Utc;

impl DiscordWhoIs {
    /// Send this query for a given discord user.
    pub async fn send(discord_id: u64) -> Result<ResponseData, TelescopeError> {
        // Construct the query variables
        let query_vars = Variables {
            now: Utc::today().naive_utc(),
            discord_id: format!("{}", discord_id)
        };

        // Send the query.
        return send_query::<Self>(query_vars).await;
    }
}
