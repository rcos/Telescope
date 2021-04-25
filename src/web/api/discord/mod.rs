//! Discord API utilities and serenity tie-ins.

mod event_handler;
use event_handler::Handler;

use serenity::client::Client;
use actix::{Actor, ActorContext, Context, AsyncContext, ActorFuture};
use crate::env::{global_config, DiscordConfig};
use serenity::model::interactions::{Interaction, ApplicationCommandOptionType};
use serenity::builder::CreateInteractionOption;
use std::pin::Pin;
use std::task::Poll;
use futures::Future;

/// Zero-sized type to initialize serenity in an actix future.
struct InitSerenityFuture<F: Future<Output = Client> + std::marker::Unpin + 'static> {
    inner: F
}

impl<F: futures::Future<Output = Client> + std::marker::Unpin> ActorFuture<DiscordActor> for InitSerenityFuture<F>
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, srv: &mut DiscordActor, _: &mut <DiscordActor as Actor>::Context, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        // Get a pin on the mutable pointer to the initialization future.
        let inner: Pin<&mut F> = Pin::new(&mut self.inner);

        // Poll the inner future.
        match Future::poll(inner, cx) {
            // If the inner future is ready, add the client to the actor and return ready.
            Poll::Ready(serenity_client) => {
                srv.serenity_client = Some(serenity_client);
                return Poll::Ready(());
            },

            // Otherwise, keep waiting on the internal future.
            Poll::Pending => Poll::Pending
        }
    }
}

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
        let fut = Box::pin(init_serenity());
        // Wrap the future into an actix future.
        let actix_future = InitSerenityFuture {inner: fut};

        // Execute the future on this actor's context.
        ctx.wait(actix_future);
    }
}

