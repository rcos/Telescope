//! Discord slash command to get information about a user.

use crate::api::rcos::users::discord_whois::DiscordWhoIs;
use crate::discord_bot::commands::InteractionResult;
use crate::env::global_config;
use crate::web::profile_for;
use serenity::builder::{CreateApplicationCommand, CreateApplicationCommandOption, CreateEmbed};
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::{
    application_command::ApplicationCommandOptionType, InteractionResponseType,
};
use serenity::utils::Color;
use serenity::Result as SerenityResult;

/// The name of this slash command.
pub const COMMAND_NAME: &'static str = "whois";

/// The name of the only option available on this command.
pub const OPTION_NAME: &'static str = "user";

/// The embed color of /whois error responses.
pub const ERROR_COLOR: Color = Color::new(0xDC3545); // bootstrap 4 error color

/// Build the option for the /whois command.
fn whois_option(obj: &mut CreateApplicationCommandOption) -> &mut CreateApplicationCommandOption {
    obj.name(OPTION_NAME)
        .kind(ApplicationCommandOptionType::User)
        .description("The user to get information about")
        .required(true)
}

/// Modify a builder object to add the info for the /whois command.
pub fn create_whois(obj: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    obj.name(COMMAND_NAME)
        .description("Get information about a member of RCOS")
        .create_option(whois_option)
}

/// Handle a user calling the /whois command from Discord.
pub fn handle_whois<'a>(
    ctx: &'a Context,
    interaction: &'a ApplicationCommandInteraction,
) -> InteractionResult<'a> {
    // Wrap the inner async function in a pinned box.
    return Box::pin(async move { handle(ctx, interaction).await });
}

/// Inner async fn to handle /whois commands without dealing with annoying types.
async fn handle(ctx: &Context, interaction: &ApplicationCommandInteraction) -> SerenityResult<()> {
    // Extract the user ID from the payload.
    let user_id = interaction
        .data
        .options
        .get(0)
        // Check that the option name matches the one set previously
        .filter(|opt| opt.name == OPTION_NAME)
        // Extract the value from the option
        .and_then(|opt| opt.value.as_ref())
        // The value should be a string containing a user ID. Extract the string
        .and_then(|val| val.as_str())
        // Then parse the user ID to a u64
        .and_then(|string| string.parse::<u64>().ok())
        // Log an error if the command has no user.
        .ok_or_else(|| {
            error!(
                "'/whois' command missing user option. Interaction: {:#?}",
                interaction
            );
        })
        // Unwrap because we expect discord not to give bad data.
        .unwrap();

    // Lookup this user on the RCOS API.
    let rcos_api_response = DiscordWhoIs::send(user_id)
        .await
        // Log the error if there is one.
        .map_err(|err| {
            error!("Could not query the RCOS API: {}", err);
            err
        });

    // Respond with an embed indicating an error on RCOS API error.
    if let Err(err) = rcos_api_response {
        return interaction
            .create_interaction_response(&ctx.http, |create_response| {
                create_response
                    // Sent the response to be a message
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    // Set the content of the message.
                    .interaction_response_data(|rdata| {
                        rdata
                            // Do not allow any mentions
                            .allowed_mentions(|am| am.empty_parse())
                            .create_embed(|embed| {
                                // Add common attributes
                                embed_common(embed)
                                    .color(ERROR_COLOR)
                                    .title("RCOS API Error")
                                    .description(
                                        "We could not get data about this user because the \
                                RCOS API responded with an error. Please contact a coordinator and \
                                report this error on Telescope's GitHub.",
                                    )
                                    // Include the error as a field of the embed.
                                    .field("Error Message", err, false)
                            })
                    })
            })
            .await;
    }

    // Error handled -- unwrap API response.
    let rcos_user: Option<_> = rcos_api_response.unwrap().get_user();

    // Respond to the discord interaction.
    return interaction
        .create_interaction_response(&ctx.http, |create_response| {
            create_response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|rdata| {
                    rdata
                        // Allow no mentions
                        .allowed_mentions(|am| am.empty_parse())
                        .create_embed(|create_embed| {
                            // Set common embed fields (author, footer, timestamp)
                            embed_common(create_embed);

                            // Set remaining fields based on user
                            if let Some(u) = rcos_user {
                                create_embed
                                    // Title with the user's name
                                    .title(format!("{} {}", u.first_name, u.last_name))
                                    // Link to their profile
                                    .url(format!(
                                        "{}{}",
                                        global_config().discord_config.telescope_url,
                                        profile_for(u.username.as_str())
                                    ))
                                    // List their role inline
                                    .field("User Role", u.role, true);

                                // Add their RPI email if available
                                let rcs_id = u
                                    .rcs_id
                                    .get(0)
                                    .map(|o| format!("{}@rpi.edu", o.account_id))
                                    .unwrap_or("RPI CAS not linked to this user.".into());

                                create_embed.field("RPI Email", rcs_id, true)
                            } else {
                                create_embed
                                    .color(ERROR_COLOR)
                                    .description("User not found in RCOS database.")
                            }
                        })
                })
        })
        .await;
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
