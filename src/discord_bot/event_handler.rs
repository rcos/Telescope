//! Event handling code for the telescope Discord Bot.

use serenity::client::EventHandler;

/// ZST representing the event handler for telescope's discord bot.
pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {

}
