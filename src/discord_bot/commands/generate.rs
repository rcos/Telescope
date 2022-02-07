//! Discord slash command to generate channels, categories and roles for small groups, projects, and project ptches.
//! Limited to coordinators, faculty advisors, and sysadmins.


// TODO: Limited to certain role
// TODO: handle event

use crate::api::rcos::users::discord_generate::DiscordGenerate;
use crate::discord_bot::commands::InteractionResult;
use crate::env::global_config;
use serenity::builder::{CreateApplicationCommand, CreateApplicationCommandOption,
    CreateApplicationCommandPermissions, CreateApplicationCommandPermissionData, CreateEmbed};
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::{
    application_command::ApplicationCommandOptionType, application_command::CreateApplicationCommandPermissionType
    ,InteractionResponseType,
};
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use serenity::utils::Color;
use serenity::Result as SerenityResult;·

// The name of this slash command.
pub const COMMAND_NAME: &'static str = "generate";
pub const OPTION_NAME: [&'static str; 4] = ["channels", "roles", "categories", "all"];
pub const ROLE: [&'static str; 2] = ["Faculty Advisors", "Coordinators"]
pub const ERROR_COLOR: Color = Color::new(0xE6770B);

// Build the option for the /generate command.
pub fn generate_option(obj: &mut CreateApplicationCommandOption, option: &'static str) -> &mut CreateApplicationCommandOption{
    match option{
        OPTION_NAME[0] => {
            obj.name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("generate channels for projects and/or small groups.");
        },
        OPTION_NAME[1] => {
            obj.name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("generate roles for projects and/or small groups.");
        },
        OPTION_NAME[2] => {
            obj.name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("generate categories for projects and/or small groups.");
        },
        OPTION_NAME[3] => {
            obj.name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("generate all for projects and/or small groups.");
        },
    }
    return obj;
}

/// Modify a builder object to add the info for the /generate command.
pub fn create_generate(obj: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    obj
        .name(COMMAND_NAME)
        .description("Generate channels, roles, categories or all. Limited to faculty advisor, coordinator, and sysamin.");
    for option in OPTION_NAME{
        obj
        .create_option(generate_option(option))
        .default_permission(false);
    }
    return obj;
}

// handle a user calling generate command from Discord.
pub fn handle_generate<'a>(
    ctx: &'a Context,
    interaction: &'a ApplicationCommandInteraction,
) -> InteractionResult<'a>{
    // Wrap the inner async function in a pinned box.
    return Box::pin(async move{handle(ctx, interaction).await});
}

async fn handle(ctx: &Context, interaction: &ApplicationCommandInteraction) -> SerenityResult<()>{
        // Extract the generate option from payload.
        let option_name= interaction
            .data
            .options
            .get(0)
            .filter(|opt| opt.value == None)
            .unwrap()
            .name
            .as_str();
            

}
