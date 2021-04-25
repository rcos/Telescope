//! Functionality for handling events from discord.

use serenity::client::{RawEventHandler, Context};
use serenity::model::event::{Event, InteractionCreateEvent};
use futures::future::BoxFuture;

pub struct Handler;

impl RawEventHandler for Handler {
    fn raw_event<'a, 'b>(&'a self, _: Context, event: Event) -> BoxFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        // Wrap all event handling into a future.
        return Box::pin(async move {
            match event {
                // Interactions -- These are slash commands.
                Event::InteractionCreate(event) => {
                    // Destructure the interaction data from the event data.
                    let InteractionCreateEvent { interaction, ..} = event;
                    
                },

                // All other events are ignored.
                _ => {},
            }
        });
    }
}
