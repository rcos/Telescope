//! Discord API interactions authenticated with the Telescope bot token.

use crate::env::global_config;
use crate::error::TelescopeError;
use serenity::http::Http;
use serenity::model::id::RoleId;

lazy_static! {
    static ref DISCORD_API_CLIENT: Http =
        Http::new_with_token(global_config().as_ref().discord_config.bot_token.as_str());
}

/// Get a reference to the global lazily evaluated static discord api client object.
pub fn global_discord_client() -> &'static Http {
    DISCORD_API_CLIENT.as_ref()
}

/// Get the ID of the verified role on the RCOS discord if it exists.
pub async fn rcos_discord_verified_role_id() -> Result<Option<RoleId>, TelescopeError> {
    // Get the RCOS Guild ID.
    let rcos_discord: u64 = global_config()
        .discord_config
        .rcos_guild_id()
        .ok_or(TelescopeError::ise("Malformed RCOS Guild ID."))?;

    // Get role
    Ok(global_discord_client()
        .get_guild_roles(rcos_discord)
        .await
        .map_err(|err| {
            error!("Could not get RCOS Discord Roles. Internal error: {}", err);
            TelescopeError::serenity_error(err)
        })?
        .iter()
        // We use a simple string comparison for now. We can change this to use
        // something else later on if needed.
        .find(|role| role.name == "Verified")
        // Extract the ID from the Discord Role.
        .map(|role| role.id))
}
