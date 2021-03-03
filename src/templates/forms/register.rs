//! Registration form and constants.

use crate::templates::forms::Form;
use crate::templates::forms::common::text_field::TextField;
use actix_web::HttpRequest;
use crate::error::TelescopeError;
use serenity::model::user::CurrentUser;
use hubcaps::users::AuthenticatedUser;

/// The path from the templates directory to the registration template.
const TEMPLATE_PATH: &'static str = "forms/register";

/// Text field for the user's first name.
const FNAME_FIELD: &'static str = "first_name";

/// Text field for the user's last name.
const LNAME_FIELD: &'static str = "last_name";

/// Template key for the user's discord, if it exists.
const DISCORD_KEY: &'static str = "discord_account";

/// Template key for the user's github account if it exists.
const GITHUB_KEY: &'static str = "github_account";

/// Create a first or last name field that validates on all non-empty strings.
fn make_name_field(name: impl Into<String>) -> TextField {
    // Convert the name string to an owned value so that it can be passed to
    // the closure constructor.
    let name_str: String = name.into();

    TextField::new(name_str.clone(), |input: Option<String>| -> TextField {
        // Create the resultant text field (with this same validator function).
        let mut result: TextField = make_name_field(name_str);

        // First/last name has to exits, and be longer than zero bytes.
        if let Some(name_str) = input.as_ref() {
            if !name_str.is_empty() {
                // The name field is not empty, and is therefore valid!
                result.value = Some(name_str.clone());
                result.success = Some("Looks Good!".into());
                result.is_valid = Some(true);
                return result;
            }
        }
        // On no/empty name, return invalid.
        result.error = Some("Cannot be empty".into());
        result.is_valid = Some(false);
        return result;
    })
}

/// Create registration form without user.
fn userless() -> Form {
    // Create form.
    let mut form: Form = Form::new(TEMPLATE_PATH, "Create Account");

    // Create text fields
    let first_name: TextField = make_name_field(FNAME_FIELD);
    let last_name: TextField = make_name_field(LNAME_FIELD);
    // Add them to the form
    form.add_text_field(first_name)
        .add_text_field(last_name);

    return form;
}

/// Create a registration form for an authenticated discord user.
pub fn with_discord(discord: &CurrentUser) -> Form {
    userless().with_other_key(DISCORD_KEY, discord)
}

// FIXME: when hubcaps gets a version above 0.6.2, remove this.
/// Serializable struct to store necessary information from the user's github.
/// This is a work-around for the lack of support for serializing this structure
/// in hubcaps.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct GitHubUserInfo {
    avatar_url: String,
    login: String,
    html_url: String,
}

/// Create a registration for an authenticated GitHub user.
pub fn with_github(github: &AuthenticatedUser) -> Form {
    // Get a serializable version of the data.
    let value = GitHubUserInfo {
        avatar_url: github.avatar_url.clone(),
        login: github.login.clone(),
        html_url: github.html_url.clone()
    };

    userless().with_other_key(GITHUB_KEY, value)
}
