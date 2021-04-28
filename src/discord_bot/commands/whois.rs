//! Discord slash command to get information about a user.

use serenity::model::interactions::ApplicationCommandOptionType;
use serenity::builder::{CreateInteraction, CreateInteractionOption};

/// Build the option for the /whois command.
fn whois_option(obj: &mut CreateInteractionOption) -> &mut CreateInteractionOption {
    obj.name("user")
        .kind(ApplicationCommandOptionType::User)
        .description("The user to get information about")
        .required(true)
}

/// Modify a builder object to add the info for the /whois command.
pub fn create_whois(obj: &mut CreateInteraction) -> &mut CreateInteraction {
    obj.name("whois")
        .description("Get information about a member of RCOS")
        .create_interaction_option(whois_option)
}
