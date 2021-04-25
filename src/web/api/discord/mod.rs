//! Discord API utilities and serenity tie-ins.

mod event_handler;
use event_handler::Handler;

use serenity::client::Client;
use actix::{Actor, ActorContext, Context};
use crate::env::{global_config, DiscordConfig};
use serenity::model::interactions::{Interaction, ApplicationCommandOptionType};
use serenity::builder::CreateInteractionOption;

/// Make the global serenity client to talk to discord.
/// Create all necessary interactions.
async fn init_serenity() -> Client {
    // Get the Discord config
    let discord_conf: &DiscordConfig = &global_config().discord_config;
    // Parse the application ID.
    let application_id: u64 = discord_conf
        .client_id
        .as_str()
        .parse::<u64>()
        .expect("Invalid discord application ID.");

    // Create the serenity client to talk to discord.
    let mut discord_client: Client = Client::builder(&discord_conf.bot_token)
        .raw_event_handler(Handler)
        .await
        .expect("Could not create serenity client");

    discord_client.start_autosharded()
        .await
        .expect("Could not start serenity client.");

    let command = Interaction::create_global_application_command(
        &discord_client.cache_and_http.http,
        application_id,
        |i| {
            // Create the argument object to this interaction
            let mut arg = CreateInteractionOption::default();
            arg
                .name("user")
                .description("The user to get information about.")
                .required(true)
                .kind(ApplicationCommandOptionType::User);

            // Add the command with the argument as "/whois".
            i.name("whois")
                .description("Get information about a user.")
                .add_interaction_option(arg)

    }).await.expect("Could not create application command.");

    return discord_client;
}

/// Zero-sized type representing an actix actor to talk to discord.
#[derive(Default)]
pub struct DiscordActor {
    /// The internal serenity client used to communicate with discord.
    serenity_client: Option<Client>
}

impl Actor for DiscordActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Initialize serenity client on start.

        // Make the client initialization future.
        let fut = async {
            // Start by initializing serenity.
            let serenity_client: Client = init_serenity().await;
            // And adding the serenity client to self.
            self.serenity_client = Some(serenity_client);
        };

        // Wrap the future into an actix future.
        let actix_future = actix::fut::wrap_future::<_, Self>(fut);


    }
}

