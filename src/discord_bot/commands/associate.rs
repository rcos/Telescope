//! Discord slash command to assoicate channel, category and role to small groups and/or project.
//! Limited to coordinators, faculty advisors, and sysadmins.

use crate::discord_bot::commands::InteractionResult;
use crate::env::global_config;
use serenity::builder::{CreateApplicationCommand, CreateApplicationCommandOption, CreateEmbed};
use serenity::client::Context;
use serenity::model::channel::{
    ChannelType as SerenityChannelType, PermissionOverwrite, PermissionOverwriteType,
};
use serenity::model::guild::Role;
use serenity::model::id::ChannelId;
use serenity::model::id::{GuildId, RoleId};
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::{
    application_command::ApplicationCommandOptionType, InteractionResponseType,
};
use serenity::model::permissions::Permissions;
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use serenity::utils::Color;
use serenity::Result as SerenityResult;

// The name of this slash command.
pub const COMMAND_NAME: &'static str = "associate";
pub const SUBCOMMAND_GROUP: [&'static str; 3] = ["channel", "role", "category"];
pub const SUBCOMMAND: [&'static str; 2] = ["project", "small group"];
pub const ERROR_COLOR: Color = Color::new(0xE6770B);

// Hepler function to check if user has permission to do /associate command.
pub fn has_permission(invoker: &RoleId, roles: &Vec<Role>) -> bool {
    roles.into_iter().any(|role| {
        (role.name != "@everyone" && role.id == *invoker)
            || role.has_permission(Permissions::ADMINISTRATOR)
    })
}

// Grant permission for certain users
fn generate_permission(project_role: Option<RoleId>, roles: Vec<Role>) -> Vec<PermissionOverwrite> {
    let mut overwrite = Vec::new();
    // set channel to be private

    for role in roles {
        // set channel to be private
        if role.name == "@everyone" {
            overwrite.push(PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::READ_MESSAGES,
                kind: PermissionOverwriteType::Role(role.id),
            })
            // Grant permission for Faculty Advisors, Coordinators and Sysadmins.
        } else {
            overwrite.push(PermissionOverwrite {
                allow: Permissions::all(),
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Role(role.id),
            });
        }
    }

    // If roles for the project have been generated, also grant permission for users who have the roles.
    if let Some(r) = project_role {
        overwrite.push(PermissionOverwrite {
            allow: Permissions::READ_MESSAGES
                | Permissions::SEND_MESSAGES
                | Permissions::EMBED_LINKS
                | Permissions::ATTACH_FILES
                | Permissions::READ_MESSAGE_HISTORY
                | Permissions::CONNECT
                | Permissions::SPEAK,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(r),
        });
    }
    return overwrite;
}

// Return error for interaction
async fn interaction_error(
    error_title: &str,
    error_description: &str,
    err: impl ToString,
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> SerenityResult<()> {
    return interaction
        .create_interaction_response(&ctx.http, |create_response| {
            create_response
                .kind(InteractionResponseType::ChannelMessageWithSource)
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
                                .title(error_title)
                                .description(error_description)
                                // Include the error as a field of the embed.
                                .field("Error Message", err, false)
                        })
                })
        })
        .await;
}

// Return success for interaction
async fn interaction_success(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    description: &str,
) -> SerenityResult<()> {
    return interaction
        .create_interaction_response(&ctx.http, |create_response| {
            create_response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|rdata| {
                    rdata
                        // Do not allow any mentions
                        .allowed_mentions(|am| am.empty_parse())
                        // Use the ephemeral flag to mark the response as only visible to the user who invoked it.
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                        .create_embed(|embed| {
                            // Add common attributes
                            embed_common(embed).title("OK").description(description)
                        })
                })
        })
        .await;
}

// Get roles information about @everyone, Faculty Advisors and Coordinator in the guilds.
async fn get_roles(ctx: &Context) -> Vec<Role> {
    GuildId(global_config().discord_config.rcos_guild_id())
        .roles(&ctx)
        .await
        .unwrap()
        .into_values()
        .collect::<Vec<Role>>()
        .into_iter()
        .filter(|role| {
            role.name == "@everyone"
                || role.name == "Faculty Advisors"
                || role.name == "Coordinators"
        })
        .collect()
}

// Build the option for the /associate command.
pub fn associate_option<'a>(
    obj: &'a mut CreateApplicationCommandOption,
    option: &'a str,
) -> &'a mut CreateApplicationCommandOption {
    match option {
        _ if option == SUBCOMMAND_GROUP[0] => obj
            .name(option)
            .kind(ApplicationCommandOptionType::SubCommandGroup)
            .description("associate channel to project and/or small group."),
        _ if option == SUBCOMMAND_GROUP[1] => obj
            .name(option)
            .kind(ApplicationCommandOptionType::SubCommandGroup)
            .description("associate role to project and/or small group."),
        _ if option == SUBCOMMAND_GROUP[2] => obj
            .name(option)
            .kind(ApplicationCommandOptionType::SubCommandGroup)
            .description("associate category to project and/or small group."),
        _ => obj,
    }
}

pub fn associate_sub_option<'a>(
    obj: &'a mut CreateApplicationCommandOption,
    option: &'a str,
) -> &'a mut CreateApplicationCommandOption{
    match option{
        _ if option == SUBCOMMAND[0] => obj
            .name(option)
            .kind(ApplicationCommandOptionType::SubCommand)
            .description("assoicate to project"),
        _ if option == SUBCOMMAND[1] => obj
            .name(option)
            .kind(ApplicationCommandOptionType::SubCommand)
            .description("associate to small group"),
        _ => obj,
    }
}

/// Modify a builder object to add the info for the /associate command.
pub fn create_associate(obj: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    obj
        .name(COMMAND_NAME)
        .description("associate channel, role, category to project and/or small group. Limited to faculty advisor, coordinator, and sysamin.");
    for subcommand_group in SUBCOMMAND_GROUP {
        for subcommand in SUBCOMMAND {
            obj.create_option(|obj| associate_option(obj, subcommand_group).create_sub_option( |obj| associate_sub_option(obj, subcommand)));
        }
    }
    return obj;
}

// handle a user calling associate command from Discord.
pub fn handle_associate<'a>(
    ctx: &'a Context,
    interaction: &'a ApplicationCommandInteraction,
) -> InteractionResult<'a> {
    // Wrap the inner async function in a pinned box.
    return Box::pin(async move { handle(ctx, interaction).await });
}

async fn handle(ctx: &Context, interaction: &ApplicationCommandInteraction) -> SerenityResult<()> {
    // Extract the generate option from payload.
    let subcommand_group = interaction
        .data
        .options
        .get(0)
        .filter(|opt| opt.value == None)
        .unwrap()
        .name
        .as_str();
    
    let subcommand = interaction
        .data
        .options
        .get(0)
        .filter(|opt| opt.value == None)
        .unwrap()
        .options
        .get(0)
        .filter(|opt| opt.value == None)
        .unwrap()
        .name
        .as_str();

    // Extract the roles of invoker.
    let roles = interaction.member.as_ref().unwrap().roles.as_slice();

    // Get the roles having permission to call /generate command
    let permitted_roles = get_roles(ctx).await;

    // If invoker's role does not have permission, or is not admin
    // respond with an embed indicating an error.
    if !roles.iter().any(|e| has_permission(e, &permitted_roles))
        && !interaction
            .member
            .as_ref()
            .unwrap()
            .permissions
            .unwrap()
            .administrator()
    {
        return interaction
            .create_interaction_response(&ctx.http, |create_response| {
                create_response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
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
                                    .description("You need Coordinator/Faculty Advisor role.")
                                    // Include the error as a field of the embed.
                                    .field("Error Message", "Permission Error", false)
                            })
                    })
            })
            .await;
    }
    match subcommand_group {
        _ if subcommand_group == SUBCOMMAND_GROUP[0] => {
            let result = handle_associate_channel(ctx, interaction, subcommand).await;
            match result {
                Some(x) => x,
                None => {
                    interaction_success(
                        ctx,
                        interaction,
                        "Associated channel to project and/or small group",
                    )
                    .await
                }
            }
        }
        _ if subcommand_group == SUBCOMMAND_GROUP[1] => {
            let result = handle_associate_role(ctx, interaction, subcommand).await;
            match result {
                Some(x) => x,
                None => {
                    interaction_success(
                        ctx,
                        interaction,
                        "Associated role to project and/or small group",
                    )
                    .await
                }
            }
        }
        _ if subcommand_group == SUBCOMMAND_GROUP[2] => {
            let result = handle_associate_category(ctx, interaction, subcommand).await;
            match result {
                Some(x) => x,
                None => {
                    interaction_success(
                        ctx,
                        interaction,
                        "Associated category to project and/or small group",
                    )
                    .await
                }
            }
        }
        _ => {
            return interaction
                .create_interaction_response(&ctx.http, |create_response| {
                    create_response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
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
                                        .title("Option Error")
                                        .description("Wrong option name .")
                                        // Include the error as a field of the embed.
                                        .field("Error Message", "Option Error", false)
                                })
                        })
                })
                .await;
        }
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

pub async fn handle_associate_channel(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    subcommad: &str
) -> Option<SerenityResult<()>> {
    Some(interaction_success(ctx, interaction,subcommad).await)
}


pub async fn handle_associate_role(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    subcommand: &str
) -> Option<SerenityResult<()>> {
    Some(interaction_success(ctx, interaction, subcommand).await)
}


pub async fn handle_associate_category(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    subcommand: &str
) -> Option<SerenityResult<()>> {
    Some(interaction_success(ctx, interaction, subcommand).await)
}
