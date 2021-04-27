//! Discord functionality built on top of serenity.
//!

mod event_handler;

use actix::{Actor, ActorContext, Context, Supervised, ActorFuture, AsyncContext};
use crate::env::{global_config, DiscordConfig};
use serenity::{Client, Result as SerenityResult};
use crate::discord_bot::event_handler::Handler;
use std::pin::Pin;
use std::task::Poll;
use std::task::Context as StdContext;
use futures::{
    future::BoxFuture,
    Future
};



/// Telescope's discord bot. This is instantiated once and used with
/// as an actix actor.
pub struct DiscordBot {
    serenity_client: Client
}

impl DiscordBot {
    /// Create an instance of the telescope Discord bot.
    pub async fn create() -> SerenityResult<Self> {
        // Get the global Discord config
        let discord_conf: &DiscordConfig = &global_config().discord_config;

        // Instantiate a serenity Discord client.
        let client: Client = Client::builder(&discord_conf.bot_token)
            .event_handler(Handler)
            .await?;

        // Wrap the client in this actor type.
        return Ok(Self { serenity_client: client });
    }

    /// Listen for incoming Discord events and handle them
    async fn listen(&mut self) -> SerenityResult<()> {
        self.serenity_client.start_autosharded().await
    }

    /// Listen for incoming Discord events in an actix-compatible future.
    fn listen_in_actor_fut(&mut self) -> ListeningFuture {
        // Start listening for discord events.
        let listening_fut = self.listen();
        // Box the listening future and convert it to an ActorFuture.
        let boxed = Box::pin(listening_fut);
        return ListeningFuture { inner: boxed };
    }
}

/// Future representing the indefinite async computation of the Discord bot
/// listening for events.
struct ListeningFuture {
    inner: BoxFuture<'static, SerenityResult<()>>
}

impl ActorFuture for ListeningFuture {
    type Output = ();
    type Actor = DiscordBot;

    fn poll(self: Pin<&mut Self>, srv: &mut Self::Actor, ctx: &mut <DiscordBot as Actor>::Context, task: &mut StdContext<'_>) -> Poll<Self::Output> {
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
        ctx.wait(self.listen_in_actor_fut());
    }
}

impl Supervised for DiscordBot {
    fn restarting(&mut self, ctx: &mut Self::Context) {
        // Restart the Discord bot when it crashes.
        error!("Discord bot crashed. Restarting now");
        // Start listening for Discord events again on the new context.
        ctx.wait(self.listen_in_actor_fut());
    }
}
