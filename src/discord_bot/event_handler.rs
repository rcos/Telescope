//! Event handling code for the telescope Discord Bot.

use crate::discord_bot::commands::{get_handler, register_commands_for_guild, InteractionHandler};
use crate::env::global_config;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::guild::Guild;
use serenity::model::interactions::Interaction;

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
        info!(
            "{}uild connected: {} (ID: {})",
            is_new.then(|| "NEW g").unwrap_or("G"),
            guild.name,
            guild.id
        );

        // Check if the guild is whitelisted.
        if global_config()
            .discord_config
            .guild_ids
            .contains(guild.id.as_u64())
        {
            // If so, register telescope's commands
            info!(
                "Registering telescope's Discord commands for guild \"{}\" (ID: {})",
                guild.name, guild.id
            );

            register_commands_for_guild(&mut ctx, &guild)
                .await
                .unwrap_or_else(|err| {
                    error!(
                        "Could not register a command on guild with ID {}: {}",
                        guild.id, err
                    );
                });
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        // Let us know we're connected.
        info!(
            "{} is connected! (user id: {})",
            ready.user.tag(),
            ready.user.id
        );

        // Get the list of global application commands.
        ctx.http
            .get_global_application_commands()
            .await
            // Log them on success
            .map(|list| {
                info!(
                    "{} global application commands registered: {:#?}",
                    list.len(),
                    list
                );
            })
            // Otherwise log an error message.
            .unwrap_or_else(|err| {
                error!("Could not get list of global application commands: {}", err);
            });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            // Application commands. These map to one of the commands registered
            // in the global command ID map.
            Interaction::ApplicationCommand(command) => {
                // Clone the command name.
                let command_name = command.data.name.clone();

                // Get the command's handler
                let handler: Option<InteractionHandler> = get_handler(command_name.as_str());

                // Error if the handler doesn't exist.
                if handler.is_none() {
                    error!("Handler not found for '/{}'. Command: {:#?}", command_name, command);
                    return;
                }

                // Call the handler on the interaction.
                let result: serenity::Result<()> = (handler.unwrap())(&ctx, &command).await;

                // Log any errors from the handler.
                if let Err(err) = result {
                    error!("'/{}' handler returned an error: {}", command_name, err);
                }
            }

            // Non-exhaustive match requires other branch.
            other => warn!("Unhandled interaction: {:?}", other),
        }
    }
}
