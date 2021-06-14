//! Telescope's discord bot commands.

use dashmap::DashMap;
use futures::future::BoxFuture;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::guild::Guild;
use serenity::model::interactions::{Interaction, ApplicationCommandInteractionData};
use serenity::model::prelude::ApplicationCommand;

mod whois;

/// Interactions return a boxed future of a serenity result.
type InteractionResult<'a> = BoxFuture<'a, serenity::Result<()>>;

/// Interaction handler type. All interaction handlers are references to
/// async functions that act on context and interaction data.
type InteractionHandler = for<'a> fn(&'a Context, &'a Interaction, &'a ApplicationCommandInteractionData) -> InteractionResult<'a>;

/// Command builder type. These builder function all act on serenity models
/// and add the necessary info to them for each command.
type CommandBuilder = fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand;

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
        handler: whois::handle_whois,
    },
];

// Global command map.
lazy_static! { static ref COMMAND_MAP: DashMap<String, InteractionHandler> = {
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
    // Register each command to the whitelisted Guild ID.
    for cmd in COMMANDS {
        // Create the default command application command object with no fields.
        let mut command_builder: CreateApplicationCommand = CreateApplicationCommand::default();
        // Populate the objects fields using the builder method for this command.
        (cmd.builder)(&mut command_builder);
        // Convert serenity's hashmap to a JSON map.
        let json_map = serenity::utils::hashmap_to_json_map(command_builder.0);
        // And put that map in a JSON value.
        let json_value = serde_json::Value::Object(json_map);

        // Send the HTTP request to create (or update) the guild command.
        let created: ApplicationCommand = ctx
            .http
            .create_guild_application_command(*guild.id.as_u64(), &json_value)
            .await?;

        info!(
            "Registered '/{}' command for '{}' guild (command ID: {}) (guild ID: {})",
            created.name, guild.name, created.id, guild.id
        );
    }

    return Ok(());
}
