//! Templates for users to login and register with.

use crate::templates::Template;
use crate::web::services::auth::oauth2_providers::{discord::DiscordOAuth, github::GitHubOauth};
use crate::web::services::auth::IdentityProvider;
use serde_json::{Map, Value};

/// Path to template from template directory root.
const TEMPLATE_PATH: &'static str = "auth";

/// Handlebars key to change the header of the page.
pub const HEADER: &'static str = "header";

/// Handlebars key for list of identity provider objects to display
/// on the page.
pub const ITEMS: &'static str = "items";

/// Handlebars key on identity providers for the link to take the user to.
pub const LINK: &'static str = "link";

/// Handlebars key on identity providers for the additional classes to add
/// (beyond "btn w-100").
pub const CLASS: &'static str = "class";

/// Handlebars key on identity providers for the message to display in the
/// button.
pub const MESSAGE: &'static str = "message";

/// Optional Handlebars key on identity providers for an icon to be displayed
/// in the button next to the message.
pub const ICON: &'static str = "icon";

/// New empty template with reference to the proper handlebars file.
fn empty() -> Template {
    Template::new(TEMPLATE_PATH)
}

/// Create an item to add to an auth template.
fn item(
    link: String,
    class: &'static str,
    message: impl Into<Value>,
    icon: Option<&'static str>,
) -> Map<String, Value> {
    // Create map.
    let mut m: Map<String, Value> = Map::new();
    // Insert keys.
    m.insert(LINK.into(), link.into());
    m.insert(CLASS.into(), class.into());
    m.insert(MESSAGE.into(), message.into());
    if let Some(i) = icon {
        m.insert(ICON.into(), i.into());
    }
    // Return map.
    return m;
}

/// Create a template to offer the user options to login.
pub fn login() -> Template {
    // Make list of identity providers in login configuration.
    let items: Vec<Map<String, Value>> = vec![
        item(
            GitHubOauth::login_path(),
            "btn-github mb-2",
            "Login using GitHub",
            Some("github"),
        ),
        item(
            DiscordOAuth::login_path(),
            "btn-discord",
            "Login using Discord",
            // This is manually coded for in the template file and is not
            // a Feather icon. Do not use it in other places, as it won't work.
            Some("discord"),
        ),
    ];

    // Create and return template.
    return empty().field(HEADER, "Sign In").field(ITEMS, items);
}

/// Create a template to offer the users options to register a new account.
pub fn register() -> Template {
    // Make list of identity providers in account creation configuration.
    let items: Vec<Map<String, Value>> = vec![
        item(
            GitHubOauth::register_path(),
            "btn-github mb-2",
            "Register using GitHub",
            Some("github"),
        ),
        item(
            DiscordOAuth::register_path(),
            "btn-discord",
            "Register using Discord",
            // This is manually coded for in the template file and is not
            // a Feather icon. Do not use it in other places, as it won't work.
            Some("discord"),
        ),
    ];

    // Create and return template.
    return empty().field(HEADER, "Create account").field(ITEMS, items);
}
