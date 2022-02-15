//! Discord slash command to generate channels, categories and roles for small groups, projects, and project ptches.
//! Limited to coordinators, faculty advisors, and sysadmins.


// TODO: Limited to certain role
// TODO: handle event

use crate::api::rcos::projects::discord_generate::DiscordGenerate;
use crate::discord_bot::commands::InteractionResult;
use crate::env::global_config;
use serenity::builder::{CreateApplicationCommand, CreateApplicationCommandOption, CreateEmbed};
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::{
    application_command::ApplicationCommandOptionType, InteractionResponseType,
};
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use serenity::model::id::{GuildId, RoleId};
use serenity::utils::Color;
use serenity::Result as SerenityResult;

// The name of this slash command.
pub const COMMAND_NAME: &'static str = "generate";
pub const OPTION_NAME: [&'static str; 4] = ["channels", "roles", "categories", "all"];
pub const ROLE_ID: [&'static RoleId; 2] = [&RoleId(939836159134158858), &RoleId(940751879493799936)];
pub const ERROR_COLOR: Color = Color::new(0xE6770B);


pub fn has_permission(role: & RoleId) -> bool{
    if ROLE_ID.contains(&role){
        return true;
    }
    false
}

// Build the option for the /generate command.
pub fn generate_option<'a>(obj: &'a mut CreateApplicationCommandOption, option: &'a str ) -> &'a mut CreateApplicationCommandOption{
    match option{
        _ if option == OPTION_NAME[0] => {
            obj.name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("generate channels for projects and/or small groups.")
        },
        _ if option == OPTION_NAME[1] => {
            obj.name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("generate roles for projects and/or small groups.")
        },
        _ if option == OPTION_NAME[2] => {
            obj.name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("generate categories for projects and/or small groups.")
        },
        _ if option == OPTION_NAME[3] => {
            obj.name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("generate all for projects and/or small groups.")
        },
        _ =>{
            obj
        }
    }
}

/// Modify a builder object to add the info for the /generate command.
pub fn create_generate(obj: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    obj
        .name(COMMAND_NAME)
        .description("Generate channels, roles, categories or all. Limited to faculty advisor, coordinator, and sysamin.");
    for option in OPTION_NAME{
        obj
        .create_option(|object|generate_option(object, option));
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
        
        // Extract the roles of invoker.
        let roles = interaction
            .member
            .as_ref()
            .unwrap()
            .roles
            .as_slice();
        
        // If missing correct options, or invoker's role does not have permission,
        // respond with an embed indicating an error.
        if !OPTION_NAME.contains(&option_name) ||
            !roles.iter().any(|e| has_permission(e)){
                return interaction.create_interaction_response(&ctx.http, |create_response|{
                    create_response.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|rdata| {
                            rdata
                             // Do not allow any mentions
                            .allowed_mentions(|am| am.empty_parse())
                             // Use the ephemeral flag to mark the response as only visible to the user who invoked it.
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)                            
                            .create_embed(|embed| {
                                // Add common attributes
                                embed_common(embed)
                                    .color(ERROR_COLOR)
                                    .title("Permission Error")
                                    .description(
                                        "We could not generate channels/roles/categories for you due to lack of permission."
                                    )
                                    // Include the error as a field of the embed.
                                    .field("Error Message","Permissoion Error", false)
                            })
                        })
                }).await;
            }else{
                return interaction.create_interaction_response(&ctx.http, |create_response|{
                    create_response.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|rdata| {
                            rdata
                             // Do not allow any mentions
                            .allowed_mentions(|am| am.empty_parse())
                             // Use the ephemeral flag to mark the response as only visible to the user who invoked it.
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)                            
                            .create_embed(|embed| {
                                // Add common attributes
                                embed_common(embed)
                                    .title("Ok")
                                    .description(
                                        "Ok"
                                    )

                            })
                        })
                }).await;
            }
}


/// Add common data to a Discord embed. This includes the author, footer, and timestamp.
fn embed_common(create_embed: &mut CreateEmbed) -> &mut CreateEmbed {
    create_embed
        // Timestamp is always now
        .timestamp(&chrono::Utc::now())
        // Footer is telescope version
        .footer(|create_footer| {
            create_footer.text(format!("Telescope {}", env!("CARGO_PKG_VERSION")))
        })
        // Author links to telescope's github.
        .author(|create_author| {
            create_author
                // Don't include the telescope icon - we only link to the github
                .name("Telescope")
                .url("https://github.com/rcos/Telescope")
        })
}
