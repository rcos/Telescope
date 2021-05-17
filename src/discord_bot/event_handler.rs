//! Event handling code for the telescope Discord Bot.

use crate::discord_bot::commands::{get_handler, register_commands_for_guild};
use crate::env::global_config;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::guild::Guild;
use serenity::model::interactions::{InteractionResponseType, InteractionType};
use serenity::model::prelude::Interaction;

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
            .debug_guild_ids
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
        match interaction.kind {
            // Respond to pings with a pong.
            InteractionType::Ping => {
                // Respond with pong
                let r: serenity::Result<()> = interaction
                    .create_interaction_response(ctx.http.as_ref(), |r| {
                        r.kind(InteractionResponseType::Pong)
                    })
                    .await;

                // Handle errors
                if let Err(err) = r {
                    error!("Error responding to Ping interaction: {}", err);
                }
            }

            // Application commands. These map to one of the commands registered
            // in the global command ID map.
            InteractionType::ApplicationCommand => {
                // The data field should always be available on this variant.
                let command_name = interaction.data.as_ref().map(|data| data.name.as_str());

                // If we receive a command without a name, throw an error and return.
                if command_name.is_none() {
                    error!("Received command without name: {:#?}", interaction);
                    return;
                }

                // Unwrap the existing value
                let command_name: &str = command_name.unwrap();
                // Get the command's handler
                let handler = get_handler(command_name);

                // Error if the handler doesn't exist.
                if handler.is_none() {
                    error!(
                        "Handler not found for '/{}'. Interaction: {:#?}",
                        command_name, interaction
                    );
                    return;
                }

                // Otherwise unwrap the handler and call it on the interaction.
                let handler = handler.unwrap();
                // Clone the command name first to avoid use-after-move.
                let command_name = command_name.to_string();
                // Call the handler on the interaction.
                let handler_result: serenity::Result<()> = handler(ctx, interaction).await;
                // Log any errors from the handler.
                if let Err(err) = handler_result {
                    error!("'/{}' handler returned an error: {}", command_name, err);
                }
            }

            // Non-exhaustive match requires other branch.
            other => warn!("Unhandled interaction type: {:?}", other),
        }
    }
}
