//! Discord API interactions authenticated with the Telescope bot token.

use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap};
use serenity::model::user::User;
use crate::env::global_config;
use crate::error::TelescopeError;

/// The Discord API endpoint to query for user data.
pub const DISCORD_API_ENDPOINT: &'static str = "https://discord.com/api/v8";

/// Lookup a discord user's info by their user ID snowflake.
pub async fn lookup_user(user_id: &str) -> Result<User, TelescopeError> {
    return reqwest::Client::new()
        .get(format!("{}/users/{}", DISCORD_API_ENDPOINT, user_id).as_str())
        .header(AUTHORIZATION, format!("Bot {}", global_config().discord_config.bot_token))
        .header(ACCEPT, "application/json")
        .send()
        .await
        .map_err(|e| {
            TelescopeError::ise(format!("Could not send user lookup request to discord. Internal error: {}", e))
        })?
        .json::<User>()
        .await
        .map_err(|e| {
            TelescopeError::ise(format!("Could not deserialize discord user object. Internal error: {}", e))
        })
}
