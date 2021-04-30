//! Discord slash command to get information about a user.

use crate::api::rcos::users::discord_whois::DiscordWhoIs;
use crate::discord_bot::commands::InteractionResult;
use futures::future::LocalBoxFuture;
use serenity::builder::{CreateInteraction, CreateInteractionOption};
use serenity::client::Context;
use serenity::model::interactions::{
    ApplicationCommandInteractionData, ApplicationCommandOptionType, Interaction,
};
use serenity::model::prelude::{ApplicationCommandInteractionDataOption, InteractionResponseType};
use serenity::model::user::User;
use serenity::{Error as SerenityError, Result as SerenityResult};

/// The name of this slash command.
pub const COMMAND_NAME: &'static str = "whois";

/// The name of the only option available on this command.
pub const OPTION_NAME: &'static str = "user";

/// Build the option for the /whois command.
fn whois_option(obj: &mut CreateInteractionOption) -> &mut CreateInteractionOption {
    obj.name(OPTION_NAME)
        .kind(ApplicationCommandOptionType::User)
        .description("The user to get information about")
        .required(true)
}

/// Modify a builder object to add the info for the /whois command.
pub fn create_whois(obj: &mut CreateInteraction) -> &mut CreateInteraction {
    obj.name(COMMAND_NAME)
        .description("Get information about a member of RCOS")
        .create_interaction_option(whois_option)
}

/// Handle a user calling the /whois command from Discord.
pub fn handle_whois(ctx: Context, interaction: Interaction) -> InteractionResult {
    // Wrap the inner async function in a pinned box.
    return Box::pin(async move { handle(ctx, interaction).await });
}

/// Inner async fn to handle /whois commands without dealing with annoying types.
async fn handle(ctx: Context, interaction: Interaction) -> SerenityResult<()> {
    // Get the interaction data reference
    let data: &ApplicationCommandInteractionData = interaction
        .data
        .as_ref()
        // This should exist and be checked for before now.
        .unwrap();

    // Extract the user ID from the payload.
    let user_id = data
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

    // Lookup this user on the discord and RCOS API.
    let discord_user_info: User = ctx.http.get_user(user_id).await?;
    let rcos_api_response = DiscordWhoIs::send(user_id)
        .await
        // Log the error if there is one.
        .map_err(|err| {
            error!("Could not query the RCOS API: {}", err);
            err
        });

    if let Err(err) = rcos_api_response {
        unimplemented!()
        /*
        interaction.create_interaction_response(ctx.http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|rdata| {
                    rdata.content("***Error***: Could not query RCOS API. @venus blonde!")
                        .allowed_mentions(|allow| {
                            allow.users(&[407704723618136065])
                        })
                })
        })

         */
    }

    Ok(())
}
