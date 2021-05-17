//! Discord functionality built on top of serenity.

mod commands;
mod event_handler;

use crate::discord_bot::event_handler::Handler;
use crate::env::{global_config, DiscordConfig};
use actix::{Actor, ActorContext, ActorFuture, AsyncContext, Context, Supervised};
use futures::future::LocalBoxFuture;
use futures::Future;
use serenity::{Client, Result as SerenityResult};
use std::pin::Pin;
use std::task::Context as StdContext;
use std::task::Poll;

/// ZST representing Telescope's discord bot. The actual client is stored by the
/// future representing the bots listening state.
pub struct DiscordBot;

impl DiscordBot {
    /// Create a Serenity Discord client.
    async fn create() -> SerenityResult<Client> {
        // Get the global Discord config
        let discord_conf: &DiscordConfig = &global_config().discord_config;
        // Extract the application ID.
        let app_id = discord_conf
            .client_id
            .as_str()
            .parse::<u64>()
            .expect("Could not parse Discord Application ID.");

        // Instantiate a serenity Discord client.
        return Client::builder(&discord_conf.bot_token)
            .event_handler(Handler)
            .application_id(app_id)
            .await;
    }

    /// Create a Discord client and start listening for Discord events.
    async fn create_and_listen() -> SerenityResult<()> {
        Self::create().await?.start_autosharded().await
    }

    /// Run create_and_listen in an Actix compatible future.
    fn wrapped_create_and_listen() -> ListeningFuture {
        ListeningFuture {
            inner: Box::pin(Self::create_and_listen()),
        }
    }
}

/// Future representing the indefinite async computation of the Discord bot
/// listening for events.
struct ListeningFuture {
    inner: LocalBoxFuture<'static, SerenityResult<()>>,
}

impl ActorFuture for ListeningFuture {
    type Output = ();
    type Actor = DiscordBot;

    fn poll(
        mut self: Pin<&mut Self>,
        _: &mut Self::Actor,
        ctx: &mut <DiscordBot as Actor>::Context,
        task: &mut StdContext<'_>,
    ) -> Poll<Self::Output> {
        // Pin a mutable reference to the internal future.
        let pinned_inner = Pin::new(&mut self.inner);
        // Poll the pinned internal future.
        match Future::poll(pinned_inner, task) {
            // If the internal future is pending, so is this one.
            // Unless the bot crashes, the internal future will be pending indefinitely.
            Poll::Pending => Poll::Pending,

            // If it's ready, the bot has crashed. Log an error message and stop the context.
            Poll::Ready(res) => {
                // Log the error
                error!("Serenity Discord client crashed and returned: {:?}", res);
                // Stop the context and the actor
                ctx.stop();
                // Return ready with no value.
                Poll::Ready(())
            }
        }
    }
}

impl Actor for DiscordBot {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Starting Discord bot");

        // Get the global Discord config
        let discord_conf: &DiscordConfig = &global_config().discord_config;

        // Log a link to invite the bot to a server.
        info!("Invite bot using \
        https://discord.com/api/oauth2/authorize?client_id={}&permissions=2147549184&response_type=code&scope=bot%20applications.commands",
              discord_conf.client_id.as_str());

        // Listen for incoming Discord events on this actor's context.
        ctx.wait(Self::wrapped_create_and_listen());
    }
}

impl Supervised for DiscordBot {
    fn restarting(&mut self, ctx: &mut Self::Context) {
        // Restart the Discord bot when it crashes.
        error!("Discord bot crashed. Restarting now");
        // Start listening for Discord events again on the new context.
        ctx.wait(Self::wrapped_create_and_listen());
    }
}
