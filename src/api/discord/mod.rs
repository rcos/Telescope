//! Discord API interactions authenticated with the Telescope bot token.

use crate::env::global_config;
use serenity::http::Http;

lazy_static! {
    static ref DISCORD_API_CLIENT: Http =
        Http::new_with_token(global_config().as_ref().discord_config.bot_token.as_str());
}

/// Get a reference to the global lazily evaluated static discord api client object.
pub fn global_discord_client() -> &'static Http {
    DISCORD_API_CLIENT.as_ref()
}
