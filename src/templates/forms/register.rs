//! Registration form and constants.

use crate::error::TelescopeError;
use crate::templates::forms::common::text_field::TextField;
use crate::templates::forms::Form;
use crate::web::services::auth::identity::RootIdentity;
use crate::web::api::rcos::users::UserAccountType;

/// The path from the templates directory to the registration template.
const TEMPLATE_PATH: &'static str = "forms/register";

/// Text field for the user's first name.
pub const FNAME_FIELD: &'static str = "first_name";

/// Text field for the user's last name.
pub const LNAME_FIELD: &'static str = "last_name";

/// Template key for the icon string for the platform that the user
/// authenticated on.
const ICON: &'static str = "icon";

/// Template key for the user's info.
const INFO: &'static str = "info";

/// Create a first or last name field that validates on all non-empty strings.
fn make_name_field(name: impl Into<String>) -> TextField {
    // Convert the name string to an owned value so that it can be passed to
    // the closure constructor.
    let name_str: String = name.into();

    TextField::new(name_str.clone(), |input: Option<&String>| -> TextField {
        // Create the resultant text field (with this same validator function).
        let mut result: TextField = make_name_field(name_str);

        // First/last name has to exits, and be longer than zero bytes.
        if let Some(name_str) = input {
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
        result
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
    form.add_text_field(first_name).add_text_field(last_name);

    form.submit_button.text = "Create Account".into();
    form.submit_button.class = Some("btn-success".into());

    return form;
}

/// Serializable struct to store necessary information from the user's authenticated info.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserInfo {
    avatar_url: String,
    username: String,
    profile_url: Option<String>,
}

/// Create a registration page with the appropriate information depending on
/// the user's identity.
pub async fn for_identity(cookie: &RootIdentity) -> Result<Form, TelescopeError> {
    match cookie {
        // On authenticated github
        RootIdentity::GitHub(gh) => gh
            // Get the authenticated github user account
            .get_authenticated_user()
            // Wait for API call to resolve
            .await
            // Convert to form
            .map(|gh_user| {
                userless()
                    .with_other_key(ICON, UserAccountType::GitHub)
                    .with_other_key(
                        INFO,
                        UserInfo {
                            avatar_url: gh_user.avatar_url.to_string(),
                            profile_url: Some(gh_user.url.to_string()),
                            username: gh_user.login.clone(),
                        },
                    )
            }),

        // On authenticated Discord
        RootIdentity::Discord(d) => d
            // Get authenticated user
            .get_authenticated_user()
            // Wait for API call to resolve
            .await
            // Convert to form.
            .map(|discord_user| {
                userless()
                    .with_other_key(ICON, UserAccountType::Discord)
                    .with_other_key(
                        INFO,
                        UserInfo {
                            username: discord_user.tag(),
                            avatar_url: discord_user.face(),
                            profile_url: None,
                        },
                    )
            })
    }
}
