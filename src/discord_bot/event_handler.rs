//! Event handling code for the telescope Discord Bot.

use serenity::client::{EventHandler, Context};
use serenity::model::gateway::Ready;
use crate::discord_bot::commands;
use crate::env::global_config;
use serenity::model::guild::Guild;
use crate::discord_bot::commands::register_commands_for_guild;

/// Get the global config's discord client ID parsed to a u64.
pub fn discord_client_id() -> u64 {
    global_config()
        .discord_config
        .client_id
        .parse::<u64>()
        .expect("Malformed Discord Client ID")
}

/// ZST representing the event handler for telescope's discord bot.
pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, mut ctx: Context, guild: Guild, is_new: bool) {
        info!("{}uild connected: {} (ID: {})",
              is_new.then(|| "NEW g").unwrap_or("G"), guild.name, guild.id);

        // Check if the guild is whitelisted.
        if global_config().discord_config.debug_guild_ids.contains(guild.id.as_u64()) {
            // If so, register telescope's commands
            info!("Registering telescope's Discord commands for guild \"{}\" (ID: {})",
                  guild.name, guild.id);

            register_commands_for_guild(&mut ctx, guild.id.0)
                .await
                .unwrap_or_else(|err| {
                    error!("Could not register a command on guild with ID {}", guild.id);
                });
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        // Let us know we're connected.
        info!("{}#{} is connected! (user id: {})",
            ready.user.name,
            ready.user.discriminator,
            ready.user.id);

        // Get the list of global application commands.
        let got_commands: bool = ctx.http
            .get_global_application_commands(discord_client_id())
            .await
            // Log them on success
            .map(|list| {
                info!("{} global application commands registered: {:#?}", list.len(), list);
                true
            })
            // Otherwise log an error message.
            .unwrap_or_else(|err| {
                error!("Could not get list of global application commands: {}", err);
                false
            });
    }
}
