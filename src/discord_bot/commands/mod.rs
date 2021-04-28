//! Telescope's discord bot commands.

use serenity::client::Context;
use serenity::builder::CreateInteraction;
use crate::discord_bot::event_handler::discord_client_id;
use serenity::model::interactions::Interaction;
use serenity::model::id::GuildId;
use serenity::model::prelude::ApplicationCommand;

mod whois;

/// Static list of all of the functions to build Telescope's discord slash commands.
const COMMAND_BUILDERS: &'static [fn(&mut CreateInteraction) -> &mut CreateInteraction] = &[
    whois::create_whois,
];

/// Register all telescope slash command for a whitelisted guild.
pub async fn register_commands_for_guild(ctx: &mut Context, guild_id: u64) -> serenity::Result<()> {
    // Get the discord client ID.
    let app_id: u64 = discord_client_id();

    // Register each command to the whitelisted Guild ID.
    for builder in COMMAND_BUILDERS {

        let created: ApplicationCommand = Interaction::create_guild_application_command(
            ctx.http.as_ref(),
            GuildId::from(guild_id),
            app_id,
            builder
        ).await?;

        info!("Registered {} slash command for guild ID {} (command ID: {})",
              created.name, guild_id, created.id);
    }

    return Ok(());
}
