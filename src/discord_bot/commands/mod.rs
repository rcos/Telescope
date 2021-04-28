//! Telescope's discord bot commands.

use serenity::client::Context;
use serenity::builder::CreateInteraction;
use crate::discord_bot::event_handler::discord_client_id;
use serenity::model::interactions::Interaction;
use serenity::model::id::GuildId;
use serenity::model::prelude::{ApplicationCommand, CommandId};
use dashmap::DashMap;
use std::pin::Pin;
use futures::future::BoxFuture;
use std::borrow::Borrow;
use std::ops::Deref;
use serenity::model::guild::Guild;

mod whois;

/// Interactions return a boxed future of a serenity result.
type InteractionResult = BoxFuture<'static, serenity::Result<()>>;

/// Interaction handler type. All interaction handlers are references to
/// async functions that act on context and interaction data.
type InteractionHandler = fn(Context, Interaction) -> InteractionResult;

/// Command builder type. These builder function all act on serenity models
/// and add the necessary info to them for each command.
type CommandBuilder = fn(&mut CreateInteraction) -> &mut CreateInteraction;


/// Telescope's concept of a discord command.
/// A builder function and a handler function.
struct Command {
    name: &'static str,
    builder: CommandBuilder,
    handler: InteractionHandler,
}

/// Static list of all of Telescope's Discord slash commands.
const COMMANDS: &'static [Command] = &[
    // /whois
    Command {
        name: whois::COMMAND_NAME,
        builder: whois::create_whois,
        handler: whois::handle_whois
    }
];

// Global command map.
lazy_static!{ static ref COMMAND_MAP: DashMap<String, InteractionHandler> = {
    let map = DashMap::new();

    // Add the commands to the map
    for cmd in COMMANDS {
        map.insert(cmd.name.to_string(), cmd.handler);
    }

    // Return the populated map
    map
}; }

/// Get a reference to the global command map.
fn global_command_map() -> &'static DashMap<String, InteractionHandler> {
    &COMMAND_MAP
}

/// Clear the global command map. This should only be done if the Discord bot crashes.
pub fn clear_command_map() {
    global_command_map().clear()
}

/// Get the handler for a given interaction by its command name.
pub fn get_handler(command_name: &str) -> Option<InteractionHandler> {
    global_command_map()
        // Lookup the command name
        .get(command_name)
        // Get a reference if there is an entry
        .as_ref()
        // Deref entry value
        .map(|entry| *entry.value())
}

/// Register all telescope slash command for a whitelisted guild.
pub async fn register_commands_for_guild(ctx: &mut Context, guild: &Guild) -> serenity::Result<()> {
    // Get the discord client ID.
    let app_id: u64 = discord_client_id();

    // Register each command to the whitelisted Guild ID.
    for cmd in COMMANDS {

        let created: ApplicationCommand = Interaction::create_guild_application_command(
            ctx.http.as_ref(),
            guild.id,
            app_id,
            cmd.builder
        ).await?;

        info!("Registered '/{}' command for '{}' guild (command ID: {}) (guild ID: {})",
              created.name, guild.name, created.id, guild.id);
    }

    return Ok(());
}
