//! GraphQL query to get user info to populate the embed for the `/whois` command on the
//! RCOS Discord bot.

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
pub struct DiscordWhoIs;

use discord_who_is::ResponseData;
use discord_who_is::Variables;

impl DiscordWhoIs {
    /// Send this query for a given discord user.
    pub async fn send(discord_id: u64) -> Result<ResponseData, TelescopeError> {
        // Construct the query variables
        let query_vars = Variables {
            discord_id: format!("{}", discord_id),
        };

        // Send the query.
        return send_query::<Self>(query_vars).await;
    }
}

impl ResponseData {
    /// Extract the user data (if there is a user) from the response data.
    pub fn get_user(self) -> Option<discord_who_is::DiscordWhoIsUserAccountsUser> {
        // Get the mutable lists of the returned accounts.
        let mut accounts: Vec<_> = self.user_accounts;
        // Get the last one (there should be at most one).
        return accounts
            .pop()
            // Everything is in the user field.
            .map(|account| account.user);
    }
}
