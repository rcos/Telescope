//! Discord slash command to assoicate channel, category and role to small groups and/or project.
//! Limited to coordinators, faculty advisors, and sysadmins.

use std::collections::HashMap;

use crate::api::rcos::discord_associations::project::{
    create_project_channel, create_project_role, project_info,
};
use crate::api::rcos::discord_associations::small_group::{
    create_small_group_category, create_small_group_channel, create_small_group_role,
    small_group_info,
};
use crate::api::rcos::discord_associations::ChannelType;

use crate::discord_bot::commands::InteractionResult;
use crate::env::global_config;
use serenity::builder::{CreateApplicationCommand, CreateApplicationCommandOption, CreateEmbed};
use serenity::client::Context;
use serenity::model::channel::ChannelType as SerenityChannelType;
use serenity::model::guild::Role;
use serenity::model::id::ChannelId;
use serenity::model::id::{GuildId, RoleId};
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
};
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
pub const SUBCOMMAND: [&'static str; 2] = ["project", "smallgroup"];
pub const ERROR_COLOR: Color = Color::new(0xE6770B);

// Hepler function to check if user has permission to do /associate command.
pub fn has_permission(invoker: &RoleId, roles: &Vec<Role>) -> bool {
    roles.into_iter().any(|role| {
        (role.name != "@everyone" && role.id == *invoker)
            || role.has_permission(Permissions::ADMINISTRATOR)
    })
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
    option: &str,
    subcommand_group: &str,
) -> &'a mut CreateApplicationCommandOption {
    match option {
        _ if option == SUBCOMMAND[0] => match subcommand_group {
            _ if subcommand_group == SUBCOMMAND_GROUP[0] => obj
                .name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("Associate to project")
                .add_sub_option(
                    CreateApplicationCommandOption { 0: HashMap::new() }
                        .name("channel_name")
                        .description("Assoicated channel.")
                        .kind(ApplicationCommandOptionType::Channel)
                        .clone(),
                )
                .add_sub_option(
                    CreateApplicationCommandOption { 0: HashMap::new() }
                        .name("project_id")
                        .description("associate existing channel with projects.")
                        .kind(ApplicationCommandOptionType::Integer)
                        .clone(),
                ),
            _ if subcommand_group == SUBCOMMAND_GROUP[1] => obj
                .name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("Associate to project")
                .add_sub_option(
                    CreateApplicationCommandOption { 0: HashMap::new() }
                        .name("role_name")
                        .description("Associated role name.")
                        .kind(ApplicationCommandOptionType::Role)
                        .clone(),
                )
                .add_sub_option(
                    CreateApplicationCommandOption { 0: HashMap::new() }
                        .name("project_id")
                        .description("Project to be associated with.")
                        .kind(ApplicationCommandOptionType::Integer)
                        .clone(),
                ),
            _ => obj,
        },
        _ if option == SUBCOMMAND[1] => match subcommand_group {
            _ if subcommand_group == SUBCOMMAND_GROUP[0] => obj
                .name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("Associate to small groups")
                .add_sub_option(
                    CreateApplicationCommandOption { 0: HashMap::new() }
                        .name("channel_name")
                        .description("Assoicated channel.")
                        .kind(ApplicationCommandOptionType::Channel)
                        .clone(),
                )
                .add_sub_option(
                    CreateApplicationCommandOption { 0: HashMap::new() }
                        .name("small_group_id")
                        .description("associate existing channel with small groups.")
                        .kind(ApplicationCommandOptionType::Integer)
                        .clone(),
                ),
            _ if subcommand_group == SUBCOMMAND_GROUP[1] => obj
                .name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("Associate to small groups")
                .add_sub_option(
                    CreateApplicationCommandOption { 0: HashMap::new() }
                        .name("role_name")
                        .description("role name to associate to small groups.")
                        .kind(ApplicationCommandOptionType::Role)
                        .clone(),
                )
                .add_sub_option(
                    CreateApplicationCommandOption { 0: HashMap::new() }
                        .name("small_group_id")
                        .description("small group to be asscoiated.")
                        .kind(ApplicationCommandOptionType::Integer)
                        .clone(),
                ),
            _ if subcommand_group == SUBCOMMAND_GROUP[2] => obj
                .name(option)
                .kind(ApplicationCommandOptionType::SubCommand)
                .description("Associate to small groups")
                .add_sub_option(
                    CreateApplicationCommandOption { 0: HashMap::new() }
                        .name("category_name")
                        .description("Assoicated category.")
                        .kind(ApplicationCommandOptionType::Channel)
                        .clone(),
                )
                .add_sub_option(
                    CreateApplicationCommandOption { 0: HashMap::new() }
                        .name("small_group_id")
                        .description("associate existing category with small groups.")
                        .kind(ApplicationCommandOptionType::Integer)
                        .clone(),
                ),
            _ => obj,
        },
        _ => obj,
    }
}

/// Modify a builder object to add the info for the /associate command.
pub fn create_associate(obj: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    obj.name(COMMAND_NAME)
        .description("associate channel, role, category to project and/or small group.");

    for subcommand_group in SUBCOMMAND_GROUP {
        let command_option = &mut CreateApplicationCommandOption { 0: HashMap::new() };
        let group = associate_option(command_option, subcommand_group);
        for subcommand in SUBCOMMAND {
            if subcommand_group == SUBCOMMAND_GROUP[2] && subcommand == SUBCOMMAND[0] {
                continue;
            }
            group.create_sub_option(|obj| associate_sub_option(obj, subcommand, subcommand_group));
        }
        obj.add_option(group.clone());
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
                                        .title("Discord Error")
                                        .description("Wrong option name.")
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
    subcommand: &str,
) -> Option<SerenityResult<()>> {
    // Get channel ID from payload.
    let interaction_data_option_value = interaction
        .data
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();
    let channel_id: ChannelId;
    let channel_kind: ChannelType;
    match interaction_data_option_value {
        ApplicationCommandInteractionDataOptionValue::Channel(partial_channel) => {
            channel_id = partial_channel.id;
            if partial_channel.kind == SerenityChannelType::Text {
                channel_kind = ChannelType::DiscordText;
            } else if partial_channel.kind == SerenityChannelType::Voice {
                channel_kind = ChannelType::DiscordVoice;
            } else {
                return Some(
                    interaction_error(
                        "Discord Error",
                        "Error Channel Type.",
                        "Please select a valid voice/text channel.",
                        ctx,
                        interaction,
                    )
                    .await,
                );
            }
        }
        _ => {
            return Some(
                interaction_error(
                    "Discord Error",
                    "Unable to find channel id.",
                    "Channel ID not found, please checkf the name and try again.",
                    ctx,
                    interaction,
                )
                .await,
            );
        }
    }

    // Get project/small group ID from payload.
    let id = interaction
        .data
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .options
        .get(1)
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_i64()
        .unwrap();

    match subcommand {
        // Associate existing channel to project
        _ if subcommand == SUBCOMMAND[0] => {
            let rcos_api_response = project_info::FindProject::get_by_id(id).await.map_err(|e| {
                error!("Error getting project info: {}", e);
                e
            });
            if let Err(err) = rcos_api_response {
                return Some(
                    interaction_error(
                        "RCOS API Error",
                        "We could not get data about project because the \
                RCOS API responded with an error. Please contact a coordinator and \
                report this error on Telescope's GitHub.",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            let project_info = rcos_api_response.unwrap().projects;
            if project_info.is_empty() {
                return Some(
                    interaction_error(
                        "Project Not Found",
                        "Project ID not found. Please check the ID and try again.",
                        "We could not find the project you specified. Please check the \
                project ID and try again.",
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            let project = project_info.get(0).unwrap();
            // Insert associated channel data into database.
            let insert_channel = create_project_channel::CreateOneProjectChannel::execute(
                project.project_id,
                channel_id.to_string(),
                channel_kind,
            )
            .await
            .map_err(|err| {
                error!("Could not insert associated channel into database: {}", err);
                err
            });

            if let Err(err) = insert_channel {
                return Some(
                    interaction_error(
                        "Database Error",
                        "We could not insert assoicated channel for projects into database",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
        }
        // Associate existing channel to small group
        _ if subcommand == SUBCOMMAND[1] => {
            // Get information about small group from RCOS API.
            let rcos_api_response = small_group_info::FindSmallGroup::get_by_id(id)
                .await
                .map_err(|e| {
                    error!("Error getting small group info: {}", e);
                    e
                });
            if let Err(err) = rcos_api_response {
                return Some(
                    interaction_error(
                        "RCOS API Error",
                        "We could not get data about small group because the \
                RCOS API responded with an error. Please contact a coordinator and \
                report this error on Telescope's GitHub.",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            let small_group_info = rcos_api_response.unwrap().small_groups;
            if small_group_info.is_empty() {
                return Some(
                    interaction_error(
                        "Small group Not Found",
                        "Small group ID not found. Please check the ID and try again.",
                        "We could not find the small group you specified. Please check the \
                small group ID and try again.",
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            let small_group = small_group_info.get(0);
            // Insert associated channel data into database.
            let insert_channel = create_small_group_channel::CreateOneSmallGroupChannel::execute(
                small_group.unwrap().small_group_id,
                channel_id.to_string(),
                channel_kind,
            )
            .await
            .map_err(|err| {
                error!("Could not insert associated channel into database: {}", err);
                err
            });

            if let Err(err) = insert_channel {
                return Some(
                    interaction_error(
                        "Database Error",
                        "We could not insert assoicated channel for small groups into database",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
        }
        // Unknown subcommand, return error.
        _ => {
            return Some(
                interaction_error(
                    "Command Error",
                    "Unknown subcommand",
                    "Could not find valid subcommand.",
                    ctx,
                    interaction,
                )
                .await,
            );
        }
    }
    return None;
}

pub async fn handle_associate_role(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    subcommand: &str,
) -> Option<SerenityResult<()>> {
    let interaction_data_option_value = interaction
        .data
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    let role_id: RoleId;
    match interaction_data_option_value {
        ApplicationCommandInteractionDataOptionValue::Role(role) => {
            role_id = role.id;
        }
        _ => {
            return Some(
                interaction_error(
                    "Discord Error",
                    "Unable to find role.",
                    "Role not found, please check the name and try again.",
                    ctx,
                    interaction,
                )
                .await,
            );
        }
    }

    let id = interaction
        .data
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .options
        .get(1)
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_i64()
        .unwrap();

    match subcommand {
        // Associate existing role to project
        _ if subcommand == SUBCOMMAND[0] => {
            let rcos_api_response = project_info::FindProject::get_by_id(id).await.map_err(|e| {
                error!("Error getting project info: {}", e);
                e
            });
            if let Err(err) = rcos_api_response {
                return Some(
                    interaction_error(
                        "RCOS API Error",
                        "We could not get data about project because the \
                RCOS API responded with an error. Please contact a coordinator and \
                report this error on Telescope's GitHub.",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            let project_info = rcos_api_response.unwrap().projects;
            if project_info.is_empty() {
                return Some(
                    interaction_error(
                        "Project Not Found",
                        "Project ID not found. Please check the ID and try again.",
                        "We could not find the project you specified. Please check the \
                project ID and try again.",
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            let project = project_info.get(0).unwrap();
            // Insert associated role data into database.
            let insert_role = create_project_role::CreateOneProjectRole::execute(
                project.project_id,
                role_id.to_string(),
            )
            .await
            .map_err(|err| {
                error!("Could not insert associated role into database: {}", err);
                err
            });

            if let Err(err) = insert_role {
                return Some(
                    interaction_error(
                        "Database Error",
                        "We could not insert assoicated role for projects into database",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
        }
        // Associate existing role to small group
        _ if subcommand == SUBCOMMAND[1] => {
            let rcos_api_response = small_group_info::FindSmallGroup::get_by_id(id)
                .await
                .map_err(|e| {
                    error!("Error getting small group info: {}", e);
                    e
                });
            if let Err(err) = rcos_api_response {
                return Some(
                    interaction_error(
                        "RCOS API Error",
                        "We could not get data about small group because the \
                RCOS API responded with an error. Please contact a coordinator and \
                report this error on Telescope's GitHub.",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            let small_group_info = rcos_api_response.unwrap().small_groups;
            if small_group_info.is_empty() {
                return Some(
                    interaction_error(
                        "Project Not Found",
                        "Project ID not found. Please check the ID and try again.",
                        "We could not find the project you specified. Please check the \
                project ID and try again.",
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            let small_group = small_group_info.get(0).unwrap();
            // Insert associated role data into database.
            let insert_role = create_small_group_role::CreateOneSmallGroupRole::execute(
                small_group.small_group_id,
                role_id.to_string(),
            )
            .await
            .map_err(|err| {
                error!("Could not insert associated role into database: {}", err);
                err
            });

            if let Err(err) = insert_role {
                return Some(
                    interaction_error(
                        "Database Error",
                        "We could not insert assoicated role for small groups into database",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
        }
        _ => {
            return Some(
                interaction_error(
                    "Command Error",
                    "Unknown subcommand",
                    "Could not find valid subcommand.",
                    ctx,
                    interaction,
                )
                .await,
            );
        }
    }
    return None;
}

pub async fn handle_associate_category(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    subcommand: &str,
) -> Option<SerenityResult<()>> {
    let interaction_data_option_value = interaction
        .data
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();
    let category_id: ChannelId;
    match interaction_data_option_value {
        ApplicationCommandInteractionDataOptionValue::Channel(partial_channel) => {
            category_id = partial_channel.id;
            if partial_channel.kind != SerenityChannelType::Category {
                return Some(
                    interaction_error(
                        "Channel Error",
                        "Channel is not a category.",
                        "Please select a category channel.",
                        ctx,
                        interaction,
                    )
                    .await,
                );
            }
        }
        _ => {
            return Some(
                interaction_error(
                    "Discord Error",
                    "Unable to find category id.",
                    "Category ID not found, please check the name and try again.",
                    ctx,
                    interaction,
                )
                .await,
            );
        }
    }

    // Get small_group ID from payload.
    let small_group_id = interaction
        .data
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .options
        .get(1)
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_i64()
        .unwrap();

    match subcommand {
        // Associate existing category to small group
        _ if subcommand == SUBCOMMAND[1] => {
            // Get information about small group from RCOS API.
            let rcos_api_response =
                small_group_info::FindSmallGroup::get_by_id(small_group_id.clone())
                    .await
                    .map_err(|e| {
                        error!("Error getting small group info: {}", e);
                        e
                    });
            if let Err(err) = rcos_api_response {
                return Some(
                    interaction_error(
                        "RCOS API Error",
                        "We could not get data about small group because the \
                RCOS API responded with an error. Please contact a coordinator and \
                report this error on Telescope's GitHub.",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            let small_group_info = rcos_api_response.unwrap().small_groups;
            if small_group_info.is_empty() {
                return Some(
                    interaction_error(
                        "Small group Not Found",
                        small_group_id.clone().to_string().as_str(),
                        "We could not find the small group you specified. Please check the \
                small group ID and try again.",
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            let small_group = small_group_info.get(0);
            // Insert associated channel data into database.
            let insert_category =
                create_small_group_category::CreateOneSmallGroupCategory::execute(
                    small_group.unwrap().small_group_id,
                    category_id.to_string(),
                )
                .await
                .map_err(|err| {
                    error!(
                        "Could not insert associated category into database: {}",
                        err
                    );
                    err
                });

            if let Err(err) = insert_category {
                return Some(
                    interaction_error(
                        "Database Error",
                        "We could not insert assoicated category for small groups into database",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
        }
        // Unknown subcommand, return error.
        _ => {
            return Some(
                interaction_error(
                    "Command Error",
                    "Unknown subcommand",
                    "Could not find valid subcommand.",
                    ctx,
                    interaction,
                )
                .await,
            );
        }
    }
    return None;
}
