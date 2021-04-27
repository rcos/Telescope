//! Functionality for handling events from discord.

use serenity::client::{Context, EventHandler};
use serenity::model::event::{Event, InteractionCreateEvent};
use futures::future::BoxFuture;
use serenity::model::interactions::{InteractionType, Interaction, InteractionResponseType};
use serenity::Result as SerenityResult;

/// ZST for handling events from discord.
pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {}

/*
/// Function to handle all Discord events.
async fn handle_discord_event(ctx: Context, event: Event) {
    match event {
        // Interactions -- These are slash commands.
        Event::InteractionCreate(event) => {
            // Destructure the interaction data from the event data.
            let InteractionCreateEvent { interaction, ..} = event;
            // Send it to the interaction handler.
            handle_interaction(ctx, interaction)
                .await;
        },

        // Ready event -- This event is sent when Discord is ready.
        Event::Ready(event) => {

        },

        // All other events are ignored.
        _ => {},
    }
}

/// Handle all interactions received over the Discord gateway.
async fn handle_interaction(ctx: Context, i: Interaction) -> SerenityResult<()> {
    match i.kind {
        // Respond to pings with a pong.
        InteractionType::Ping => {
            i.create_interaction_response(ctx.http, |r| {
                r.kind(InteractionResponseType::Pong)
            }).await?;
        },

        // Otherwise destructure the command invocation.
        InteractionType::ApplicationCommand => {
            dbg!(&i);
            unimplemented!()
        }

        // We should be cover but in case they add other interactions later we
        // error out.
        other => error!("Unknown interaction type: {:?}.", other),
    }

    // If we reach this we are successful.
    return Ok(());
}
*/
