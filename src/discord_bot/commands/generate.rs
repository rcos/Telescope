//! Discord slash command to generate channels, categories and roles for small groups, projects, and project ptches.
//! Limited to coordinators, faculty advisors, and sysadmins.

use crate::api::rcos::discord_associations::project::{
    create_project_channel, create_project_role, project_info,
};
use crate::api::rcos::discord_associations::small_group::{
    create_small_group_category, create_small_group_role, small_group_info,
};
use crate::api::rcos::discord_associations::ChannelType;
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
pub const COMMAND_NAME: &'static str = "generate";
pub const OPTION_NAME: [&'static str; 4] = ["channels", "roles", "categories", "all"];
pub const ERROR_COLOR: Color = Color::new(0xE6770B);

// Hepler function to check if user has permission to do /generate command.
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

// Build the option for the /generate command.
pub fn generate_option<'a>(
    obj: &'a mut CreateApplicationCommandOption,
    option: &'a str,
) -> &'a mut CreateApplicationCommandOption {
    match option {
        _ if option == OPTION_NAME[0] => obj
            .name(option)
            .kind(ApplicationCommandOptionType::SubCommand)
            .description("generate channels for projects and/or small groups."),
        _ if option == OPTION_NAME[1] => obj
            .name(option)
            .kind(ApplicationCommandOptionType::SubCommand)
            .description("generate roles for projects and/or small groups."),
        _ if option == OPTION_NAME[2] => obj
            .name(option)
            .kind(ApplicationCommandOptionType::SubCommand)
            .description("generate categories for projects and/or small groups."),
        _ if option == OPTION_NAME[3] => obj
            .name(option)
            .kind(ApplicationCommandOptionType::SubCommand)
            .description("generate all for projects and/or small groups."),
        _ => obj,
    }
}

/// Modify a builder object to add the info for the /generate command.
pub fn create_generate(obj: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    obj
        .name(COMMAND_NAME)
        .description("Generate channels, roles, categories or all. Limited to faculty advisor, coordinator, and sysamin.");
    for option in OPTION_NAME {
        obj.create_option(|object| generate_option(object, option));
    }
    return obj;
}

// handle a user calling generate command from Discord.
pub fn handle_generate<'a>(
    ctx: &'a Context,
    interaction: &'a ApplicationCommandInteraction,
) -> InteractionResult<'a> {
    // Wrap the inner async function in a pinned box.
    return Box::pin(async move { handle(ctx, interaction).await });
}

async fn handle(ctx: &Context, interaction: &ApplicationCommandInteraction) -> SerenityResult<()> {
    // Extract the generate option from payload.
    let option_name = interaction
        .data
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
    match option_name {
        _ if option_name == OPTION_NAME[0] => {
            let result = handle_generate_channels(ctx, interaction).await;
            match result {
                Some(x) => x,
                None => {
                    interaction_success(
                        ctx,
                        interaction,
                        "Channel created for projcets and/or small groups",
                    )
                    .await
                }
            }
        }
        _ if option_name == OPTION_NAME[1] => {
            let result = handle_generate_role(ctx, interaction).await;
            match result {
                Some(x) => x,
                None => {
                    interaction_success(
                        ctx,
                        interaction,
                        "Role created for projcets and/or small groups",
                    )
                    .await
                }
            }
        }
        _ if option_name == OPTION_NAME[2] => {
            let result = handle_generate_categories(ctx, interaction).await;
            match result {
                Some(x) => x,
                None => {
                    interaction_success(
                        ctx,
                        interaction,
                        "Category created for projcets and/or small groups",
                    )
                    .await
                }
            }
        }
        _ if option_name == OPTION_NAME[3] => handle_generate_all(ctx, interaction).await,
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

// handler for /generate channel commands
async fn handle_generate_channels(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Option<SerenityResult<()>> {
    // Create channel for small groups
    let rcos_api_response = small_group_info::CurrSmallGroups::get(0, None)
        .await
        .map_err(|err| {
            error!("Could not query the RCOS API: {}", err);
            err
        });

    if let Err(err) = rcos_api_response {
        return Some(
            interaction_error(
                "RCOS API Error",
                "We could not get data about small groups because the \
        RCOS API responded with an error. Please contact a coordinator and \
        report this error on Telescope's GitHub.",
                &err,
                &ctx,
                &interaction,
            )
            .await,
        );
    }

    // Get list of discord association information for small groups.
    let small_groups_associate_info = rcos_api_response.unwrap().small_groups;

    for small_group in small_groups_associate_info {
        // Get parent channel(category) from data.
        let categories = small_group.small_group_categories;
        // Get list of projects of small group.
        let small_group_projects = &small_group.small_group_projects;
        for category in categories {
            // Create Voice and Text channel under small group category
            for small_group_project in small_group_projects {
                // Create channels for small group projects if not previously created.
                if small_group_project.project.project_channels.is_empty() {
                    // Generate permission for certain groups for the channel.
                    let overwrite = if let None = small_group_project.project.project_role {
                        generate_permission(None, get_roles(ctx).await)
                    } else {
                        generate_permission(
                            Some(RoleId(
                                small_group_project
                                    .project
                                    .project_role
                                    .as_ref()
                                    .unwrap()
                                    .role_id
                                    .parse::<u64>()
                                    .unwrap(),
                            )),
                            get_roles(ctx).await,
                        )
                    };

                    // Create voice channel.
                    let voice_channel = GuildId(global_config().discord_config.rcos_guild_id())
                        .create_channel(&ctx.http, |c| {
                            c.name(&small_group_project.project.title)
                                .kind(SerenityChannelType::Voice)
                                .permissions(overwrite.clone())
                                .category(ChannelId(category.category_id.parse::<u64>().unwrap()))
                        })
                        .await
                        .map_err(|err| {
                            error!("Could not create the voice channel: {}", err);
                            err
                        });

                    // Create text channel.
                    let text_channel = GuildId(global_config().discord_config.rcos_guild_id())
                        .create_channel(&ctx.http, |c| {
                            c.name(&small_group_project.project.title)
                                .kind(SerenityChannelType::Text)
                                .permissions(overwrite.clone())
                                .category(ChannelId(category.category_id.parse::<u64>().unwrap()))
                        })
                        .await
                        .map_err(|err| {
                            error!("Could not create the text channel: {}", err);
                            err
                        });

                    if let Err(err) = voice_channel {
                        return Some(
                            interaction_error(
                                "Discord Error",
                                "We could not create voice channel for projects",
                                &err,
                                &ctx,
                                &interaction,
                            )
                            .await,
                        );
                    }

                    if let Err(err) = text_channel {
                        return Some(
                            interaction_error(
                                "Discord Error",
                                "We could not create text channel for projects",
                                &err,
                                &ctx,
                                &interaction,
                            )
                            .await,
                        );
                    }

                    // insert voice channel data into database
                    let insert_voice_channel =
                        create_project_channel::CreateOneProjectChannel::execute(
                            small_group_project.project.project_id,
                            voice_channel.unwrap().id.to_string(),
                            ChannelType::DiscordVoice,
                        )
                        .await
                        .map_err(|err| {
                            error!(
                                "Could not insert project voice channel data to into database: {}",
                                err
                            );
                            err
                        });

                    // insert text channel data into database
                    let insert_text_channel =
                        create_project_channel::CreateOneProjectChannel::execute(
                            small_group_project.project.project_id,
                            text_channel.unwrap().id.to_string(),
                            ChannelType::DiscordText,
                        )
                        .await
                        .map_err(|err| {
                            error!(
                                "Could not insert project text channel data to into database: {}",
                                err
                            );
                            err
                        });

                    if let Err(err) = insert_voice_channel {
                        return Some(
                            interaction_error(
                                "Database Error",
                                "We could not insert voice channel for projects into database",
                                &err,
                                &ctx,
                                &interaction,
                            )
                            .await,
                        );
                    }
                    if let Err(err) = insert_text_channel {
                        return Some(
                            interaction_error(
                                "Database Error",
                                "We could not insert text channel for projects into database",
                                &err,
                                &ctx,
                                &interaction,
                            )
                            .await,
                        );
                    }
                }
            }
        }
    }
    return None;
}

// handler for /generate role commands
async fn handle_generate_role(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Option<SerenityResult<()>> {
    let rcos_api_response_project = project_info::CurrProjects::get(0, None)
        .await
        .map_err(|err| {
            error!("Could not query the RCOS API: {}", err);
            err
        });
    if let Err(err) = rcos_api_response_project {
        return Some(
            interaction_error(
                "RCOS API Error",
                "We could not get data about projects because the \
        RCOS API responded with an error. Please contact a coordinator and \
        report this error on Telescope's GitHub.",
                &err,
                &ctx,
                &interaction,
            )
            .await,
        );
    }
    let projects_associate_info = rcos_api_response_project.unwrap().projects;
    // Create role for project if is not previously set.
    for project in projects_associate_info {
        if project.project_role.is_none() {
            let role = GuildId(global_config().discord_config.rcos_guild_id())
                .create_role(&ctx.http, |r| r.name(&project.title).mentionable(true))
                .await
                .map_err(|err| {
                    error!("Could not create the channel: {}", err);
                    err
                });
            if let Err(err) = role {
                return Some(
                    interaction_error(
                        "Discord Error",
                        "We could not create role for projects",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            // Insert project role data into database.
            let insert_role = create_project_role::CreateOneProjectRole::execute(
                project.project_id,
                role.unwrap().id.to_string(),
            )
            .await
            .map_err(|err| {
                error!(
                    "Could not insert project role data to into database: {}",
                    err
                );
                err
            });

            if let Err(err) = insert_role {
                return Some(
                    interaction_error(
                        "Database Error",
                        "We could not insert role for projects into database",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
        }
    }
    let rcos_api_response_small_group = small_group_info::CurrSmallGroups::get(0, None)
        .await
        .map_err(|err| {
            error!("Could not query the RCOS API: {}", err);
            err
        });
    if let Err(err) = rcos_api_response_small_group {
        return Some(
            interaction_error(
                "RCOS API Error",
                "We could not get data about small groups because the \
        RCOS API responded with an error. Please contact a coordinator and \
        report this error on Telescope's GitHub.",
                &err,
                &ctx,
                &interaction,
            )
            .await,
        );
    }
    let small_groups_associate_info = rcos_api_response_small_group.unwrap().small_groups;
    // Create role for project if is not previously set.
    for small_group in small_groups_associate_info {
        let small_group_projects = &small_group.small_group_projects;
        if small_group.small_group_role.is_none() {
            let role = GuildId(global_config().discord_config.rcos_guild_id())
                .create_role(&ctx.http, |r| {
                    r.name(small_group.title.clone()).mentionable(true)
                })
                .await
                .map_err(|err| {
                    error!("Unable to  create the role: {}", err);
                    err
                });
            if let Err(err) = role {
                return Some(
                    interaction_error(
                        "Discord Error",
                        "We could not create role for small groups",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
            // Insert small group role data into database.
            let insert_role = create_small_group_role::CreateOneSmallGroupRole::execute(
                small_group.small_group_id,
                role.unwrap().id.to_string(),
            )
            .await
            .map_err(|err| {
                error!(
                    "Could not insert small group role data to into database: {}",
                    err
                );
                err
            });

            if let Err(err) = insert_role {
                return Some(
                    interaction_error(
                        "Database Error",
                        "We could not insert role for small groups into database",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
        }
        // Create role for small group projects if is not previously set.
        for small_group_project in small_group_projects {
            if small_group_project.project.project_role.is_none() {
                let role = GuildId(global_config().discord_config.rcos_guild_id())
                    .create_role(&ctx.http, |r| {
                        r.name(small_group_project.project.title.clone())
                            .mentionable(true)
                    })
                    .await
                    .map_err(|err| {
                        error!("Unable to  create the role: {}", err);
                        err
                    });
                if let Err(err) = role {
                    return Some(
                        interaction_error(
                            "Discord Error",
                            "We could not create role for small group projects",
                            &err,
                            &ctx,
                            &interaction,
                        )
                        .await,
                    );
                }
                let insert_role = create_project_role::CreateOneProjectRole::execute(
                    small_group_project.project.project_id,
                    role.unwrap().id.to_string(),
                )
                .await
                .map_err(|err| {
                    error!(
                        "Could not insert small group role data to into database: {}",
                        err
                    );
                    err
                });

                if let Err(err) = insert_role {
                    return Some(
                        interaction_error(
                            "Database Error",
                            "We could not insert role for small groups into database",
                            &err,
                            &ctx,
                            &interaction,
                        )
                        .await,
                    );
                }
            }
        }
    }
    return None;
}

// handler for /generate categories commands
async fn handle_generate_categories(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Option<SerenityResult<()>> {
    let rcos_api_response = small_group_info::CurrSmallGroups::get(0, None)
        .await
        .map_err(|err| {
            error!("Could not query the RCOS API: {}", err);
            err
        });

    if let Err(err) = rcos_api_response {
        return Some(
            interaction_error(
                "RCOS API Error",
                "We could not get data about small groups because the \
        RCOS API responded with an error. Please contact a coordinator and \
        report this error on Telescope's GitHub.",
                &err,
                &ctx,
                &interaction,
            )
            .await,
        );
    }

    let small_groups_associate_info = rcos_api_response.unwrap().small_groups;
    // Create category for projects if not previously created.
    for small_group in small_groups_associate_info {
        if small_group.small_group_categories.is_empty() {
            // Generate permission for certain groups for the channel.
            let overwrite = if let true = small_group.small_group_role.is_none() {
                generate_permission(None, get_roles(ctx).await)
            } else {
                generate_permission(
                    Some(RoleId(
                        small_group
                            .small_group_role
                            .unwrap()
                            .role_id
                            .parse::<u64>()
                            .unwrap(),
                    )),
                    get_roles(ctx).await,
                )
            };

            let category = GuildId(global_config().discord_config.rcos_guild_id())
                .create_channel(&ctx.http, |c| {
                    c.name(small_group.title)
                        .kind(SerenityChannelType::Category)
                        .permissions(overwrite)
                })
                .await
                .map_err(|err| {
                    error!("Could not create the category for small group: {}", err);
                    err
                });

            if let Err(err) = category {
                return Some(
                    interaction_error(
                        "Discord Error",
                        "We could not create category for small group",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }

            // insert category data into database
            let insert_category =
                create_small_group_category::CreateOneSmallGroupCategory::execute(
                    small_group.small_group_id,
                    category.unwrap().id.to_string(),
                )
                .await
                .map_err(|err| {
                    error!(
                        "Could not insert small group category data to into database: {}",
                        err
                    );
                    err
                });

            if let Err(err) = insert_category {
                return Some(
                    interaction_error(
                        "Database Error",
                        "We could not insert category for small group into database",
                        &err,
                        &ctx,
                        &interaction,
                    )
                    .await,
                );
            }
        }
    }
    return None;
}

async fn handle_generate_all(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> SerenityResult<()> {
    let generate_role = handle_generate_role(ctx, interaction).await;
    if generate_role.is_some() {
        return generate_role.unwrap();
    }
    let generate_categories = handle_generate_categories(ctx, interaction).await;
    if generate_categories.is_some() {
        return generate_categories.unwrap();
    }
    let handle_generate_channel = handle_generate_channels(ctx, interaction).await;
    if handle_generate_channel.is_some() {
        return handle_generate_channel.unwrap();
    }
    interaction_success(
        ctx,
        interaction,
        "Categories, roles, and channels are created for project and/or small groups.",
    )
    .await
}
