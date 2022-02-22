//! Discord slash command to generate channels, categories and roles for small groups, projects, and project ptches.
//! Limited to coordinators, faculty advisors, and sysadmins.


// TODO: Limited to certain role
// TODO: handle event

//use crate::api::rcos::discord_assoications::channels::ProjectChannels;
use crate::api::rcos::projects::projects_page;
use crate::api::rcos::discord_assoications::{project_info, create_channel, create_role, ChannelType};
use crate::discord_bot::commands::InteractionResult;
use crate::env::global_config;
use serenity::builder::{CreateApplicationCommand, CreateApplicationCommandOption, CreateEmbed, CreateChannel};
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::{
    application_command::ApplicationCommandOptionType, InteractionResponseType,
};
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use serenity::model::id:: {RoleId, GuildId};
use serenity::utils::Color;
use serenity::Result as SerenityResult;
use serenity::model::channel::{ChannelType as SerenityChannelType, GuildChannel, PermissionOverwrite,
    PermissionOverwriteType};
use  serenity::model::permissions::Permissions;
use serenity::model::guild::Guild;
use serenity::model::id::ChannelId;
use serenity::cache::Cache;

// The name of this slash command.
pub const COMMAND_NAME: &'static str = "generate";
pub const OPTION_NAME: [&'static str; 4] = ["channels", "roles", "categories", "all"];
// Id of the @everyone role.
pub const EVERYONE: &'static RoleId = &RoleId(939274049824129026);
// Id of the role who have permission to use /generate slash command.
pub const ROLE_ID: [&'static RoleId; 2] = [&RoleId(939836159134158858), &RoleId(940751879493799936)];
pub const ERROR_COLOR: Color = Color::new(0xE6770B);

// Hepler function to check if user has permission to do /generate command.
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
        
        // If invoker's role does not have permission,
        // respond with an embed indicating an error.
        if !roles.iter().any(|e| has_permission(e)){
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
            }
        match option_name{
            _ if option_name == OPTION_NAME[0] => {
                handle_generate_channel(ctx, interaction).await
            }
            _ if option_name == OPTION_NAME[1] => {
                handle_generate_role(ctx, interaction).await
            }
            _ if option_name == OPTION_NAME[2] => {
                handle_generate_categories(ctx, interaction).await
            }
            _ if option_name == OPTION_NAME[3] => {
                return interaction.create_interaction_response(&ctx.http, |create_response|{
                    create_response.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|rdata| {
                            rdata
                                .content("Not implemented.")
                        })
                }).await;
            }
            _ =>{
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
                                    .title("Option Error")
                                    .description(
                                        "Wrong option name ."
                                    )
                                    // Include the error as a field of the embed.
                                    .field("Error Message","Option Error", false)
                            })
                        })
                }).await;
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
 async fn handle_generate_channel(ctx: &Context, interaction: &ApplicationCommandInteraction) -> SerenityResult<()>{
    let rcos_api_response = project_info::Projects::get(0, None)
    .await
    .map_err(|err| {
    error!("Could not query the RCOS API: {}", err);
    err
});

if let Err(err) = rcos_api_response{
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
                            .title("RCOS API Error")
                            .description(
                                "We could not get data about projects because the \
                                RCOS API responded with an error. Please contact a coordinator and \
                                report this error on Telescope's GitHub.",)
                             // Include the error as a field of the embed.
                            .field("Error Message",err, false)
                    })
            })
        }).await;
}

// Get list of discord association information for projects.
let projects_associate_info = rcos_api_response.unwrap().projects;

// Create channel for projects if not previously created.
for project in projects_associate_info{
    if project.project_channels.is_empty(){
        // Generate permission for certain groups for the channel.
        let overwrite = if let true =  project.project_role.is_none(){
            generate_permission(None)
        }else{
            generate_permission(Some(RoleId(project.project_role.unwrap().role_id.parse::<u64>().unwrap())))
        };

        let channel = GuildId(global_config().discord_config.rcos_guild_id()).create_channel(&ctx.http, |c|{
            c.name(&project.title)
            .kind(SerenityChannelType::Text)
            .permissions(overwrite)
        }).await
            .map_err(|err| {
                error!("Could not create the channel: {}", err);
                err
            });

        if let Err(err) = channel{
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
                                .title("Discord Error")
                                .description(
                                    "We could not create channel for projects",)
                                    // Include the error as a field of the embed.
                                    .field("Error Message",err, false)
                            })
                    })
            }).await;
        }

        // insert channel data into database
        let insert_channel = create_channel::CreateOneChannel::execute(
                project.project_id, 
                channel.unwrap().id.to_string(),
                 ChannelType::DiscordVoice)
                 .await
                .map_err(|err|{
                    error!("Could not insert project channel data to into database: {}", err);
                    err
                });

        if let Err(err) = insert_channel{
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
                                .title("Database Error")
                                .description(
                                    "We could not insert channel for projects into database",)
                                    // Include the error as a field of the embed.
                                    .field("Error Message",err, false)
                            })
                    })
            }).await;
        }
    }
}


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
                            .title("OK")
                            .description("Channel created for projects")
                    })
            })
    }).await;
}

// handler for /generate role commands
async fn handle_generate_role(ctx: &Context, interaction: &ApplicationCommandInteraction) -> SerenityResult<()>{
    let rcos_api_response = project_info::Projects::get(0, None)
    .await
    .map_err(|err| {
    error!("Could not query the RCOS API: {}", err);
    err
});
    if let Err(err) = rcos_api_response{
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
                            .title("RCOS API Error")
                            .description(
                                "We could not get data about projects because the \
                                RCOS API responded with an error. Please contact a coordinator and \
                                report this error on Telescope's GitHub.",)
                             // Include the error as a field of the embed.
                            .field("Error Message",err, false)
                    })
            })
        }).await;
    }
    let projects_associate_info = rcos_api_response.unwrap().projects;
    // Create role for project if is not previously set.
    for project in projects_associate_info{
        if project.project_role.is_none(){
            let role = GuildId(global_config().discord_config.rcos_guild_id()).create_role(&ctx.http, |r|{
                r.name(&project.title)
                    .mentionable(true)
            }).await
                .map_err(|err| {
                    error!("Could not create the channel: {}", err);
                    err
                });
            if let Err(err) = role{
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
                                .title("Discord Error")
                                .description("We could not create role for projects",)
                                // Include the error as a field of the embed.
                                .field("Error Message",err, false)
                            })
                     })
                }).await;
            }
            let insert_role = create_role::CreateOneRole::execute(project.project_id , role.unwrap().to_string())
                .await
                .map_err(|err|{
                    error!("Could not insert project role data to into database: {}", err);
                    err
                });
            
            if let Err(err) = insert_role{
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
                                    .title("Database Error")
                                    .description(
                                        "We could not insert role for projects into database",)
                                        // Include the error as a field of the embed.
                                        .field("Error Message",err, false)
                                })
                        })
                }).await;
            }
        }
        /*
        for member in project.enrollments.user_id{
            
        }
        */
    }

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
                        .title("OK")
                        .description("Role created for projects")
                })
            })
    }).await;
}

// TODO
// handler for /generate categories commands
async fn handle_generate_categories(ctx: &Context, interaction: &ApplicationCommandInteraction) -> SerenityResult<()>{
    let rcos_api_response = project_info::Projects::get(0, None)
    .await
    .map_err(|err| {
    error!("Could not query the RCOS API: {}", err);
    err
});

if let Err(err) = rcos_api_response{
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
                            .title("RCOS API Error")
                            .description(
                                "We could not get data about projects because the \
                                RCOS API responded with an error. Please contact a coordinator and \
                                report this error on Telescope's GitHub.",)
                             // Include the error as a field of the embed.
                            .field("Error Message",err, false)
                    })
            })
        }).await;
}

// Get list of discord association information for projects.
let projects_associate_info = rcos_api_response.unwrap().projects;

// Create channel for projects if not previously created.
for project in projects_associate_info{
    if project.project_channels.is_empty(){
        // Generate permission for certain groups for the channel.
        let overwrite = if let true =  project.project_role.is_none(){
            generate_permission(None)
        }else{
            generate_permission(Some(RoleId(project.project_role.unwrap().role_id.parse::<u64>().unwrap())))
        };

        let category = GuildId(global_config().discord_config.rcos_guild_id()).create_channel(&ctx.http, |c|{
            c.name(&project.title)
            .kind(SerenityChannelType::Category)
            .permissions(overwrite)
        }).await
            .map_err(|err| {
                error!("Could not create the category: {}", err);
                err
            });

        if let Err(err) = category{
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
                                .title("Discord Error")
                                .description(
                                    "We could not create category for projects",)
                                    // Include the error as a field of the embed.
                                    .field("Error Message",err, false)
                            })
                    })
            }).await;
        }

        // insert channel data into database
        let insert_channel = create_channel::CreateOneChannel::execute(
                project.project_id, 
                category.unwrap().id.to_string(),
                 ChannelType::DiscordVoice)
                 .await
                .map_err(|err|{
                    error!("Could not insert project category data to into database: {}", err);
                    err
                });

        if let Err(err) = insert_channel{
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
                                .title("Database Error")
                                .description(
                                    "We could not insert category for projects into database",)
                                    // Include the error as a field of the embed.
                                    .field("Error Message",err, false)
                            })
                    })
            }).await;
        }
    }
}


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
                            .title("OK")
                            .description("Category created for projects")
                    })
            })
    }).await;
}
/*
async fn handle_generate_all(ctx: &Context, interaction: &ApplicationCommandInteraction) -> SerenityResult<()>{
}
*/


// Grant permission for certain users
fn generate_permission(project_role: Option<RoleId>) -> Vec<PermissionOverwrite>{
    // set channel to be private
     let mut overwrite = vec![PermissionOverwrite{
        allow: Permissions::empty(),
        deny: Permissions::READ_MESSAGES,
         kind: PermissionOverwriteType::Role(*EVERYONE),
    }];

    // Grant permission for faculty and advisor.
    for role in ROLE_ID{
        overwrite.push(PermissionOverwrite{
            allow: Permissions::all(),
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(*role)
        });
    }
    
    // If roles for the project have been generated, also grant permission for users who have the roles.
    if let Some(r) = project_role{
        overwrite.push(PermissionOverwrite{
            allow: 
            Permissions:: READ_MESSAGES |
            Permissions::SEND_MESSAGES | 
            Permissions::EMBED_LINKS | 
            Permissions:: ATTACH_FILES | 
            Permissions::READ_MESSAGE_HISTORY |
            Permissions:: CONNECT|
            Permissions:: SPEAK,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(r)
        });
    }
    return overwrite;
}

