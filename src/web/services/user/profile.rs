//! Profile services.

use crate::api::discord::{self, global_discord_client};
use crate::api::rcos::users::edit_profile::{EditProfileContext, SaveProfileEdits};
use crate::api::rcos::users::profile::{
    profile::{ProfileTarget, ResponseData},
    Profile,
};
use crate::api::rcos::users::UserRole;
use crate::env::global_config;
use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use crate::templates::Template;

use crate::web::services::auth::identity::{AuthenticationCookie, Identity};
use actix_web::web::{Form, Path, ServiceConfig};
use actix_web::{http::header::LOCATION, HttpRequest, HttpResponse};
use chrono::{Datelike, Local};
use serenity::model::guild::Member;
use serenity::model::user::User;
use std::collections::HashMap;
use uuid::Uuid;

/// The path from the template directory to the profile template.
const TEMPLATE_NAME: &'static str = "user/profile";

/// The path from the templates directory to the user settings form template.
const SETTINGS_FORM: &'static str = "user/settings";

/// Register services into actix app.
pub fn register(config: &mut ServiceConfig) {
    config
        .service(profile)
        .service(settings)
        .service(save_changes);
}

/// User profile service. The target's user ID is in the path.
#[get("/user/{id}")]
async fn profile(
    req: HttpRequest,
    identity: Identity,
    Path(id): Path<Uuid>,
) -> Result<Template, TelescopeError> {
    // Get the viewer's user ID.
    let viewer: Option<Uuid> = identity.get_user_id().await?;

    // Get the user's profile information (and viewer info) from the RCOS API.
    let response: ResponseData = Profile::for_user(id, viewer).await?;

    // Throw an error if there is no user.
    if response.target.is_none() {
        return Err(TelescopeError::resource_not_found(
            "User Not Found",
            "Could not find a user by this user ID.",
        ));
    }

    // Create the profile template to send back to the viewer.
    let mut template: Template = Template::new(TEMPLATE_NAME).field("data", &response);

    // Get the target user's info.
    let target_user: &ProfileTarget = response.target.as_ref().unwrap();
    // And use it to make the page title
    let page_title: String = format!("{} {}", target_user.first_name, target_user.last_name);

    // Get the target user's discord info.
    let target_discord_id: Option<&str> = target_user
        .discord
        .first()
        .map(|obj| obj.account_id.as_str());

    // If the discord ID exists, and is properly formatted.
    if let Some(target_discord_id) = target_discord_id.and_then(|s| s.parse::<u64>().ok()) {
        // Get target user info.
        let target_user: Result<User, serenity::Error> =
            global_discord_client().get_user(target_discord_id).await;

        // Check to make sure target user info was available.
        match target_user {
            // Issue retrieving target user info
            Err(e) => {
                // Log an error and set a flag for the template.
                warn!("Could not get target user account for Discord user ID {}. Account may have been deleted. Internal error: {}", target_discord_id, e);
                template["discord"]["target"] = json!({"errored": true});

                // Return early if there was an error.
                // Otherwise we can go forward and check for the user's membership in the RCOS
                // Discord server.
                return template.render_into_page(&req, page_title).await;
            }

            // User returned successfully.
            Ok(u) => {
                // Add the discord info to the template.
                template["discord"]["target"] = json!({
                    "response": &u,
                    "resolved": {
                        "face": u.face(),
                        "tag": u.tag(),
                    }
                });
            }
        }

        // Check if the target user is in the RCOS Discord.
        // First parse the RCOS Discord Guild ID.
        let rcos_discord: u64 = global_config().discord_config.rcos_guild_id();

        // Target user as member of RCOS discord.
        let membership: Option<Member> = global_discord_client()
            .get_member(rcos_discord, target_discord_id)
            .await
            .ok();

        // Get "Verified" role ID if available.
        let verified_role_id = discord::rcos_discord_verified_role_id()
            .await
            .ok()
            .flatten();

        // Check if the user has the verified role.
        let is_verified: bool = membership
            // Take as reference
            .as_ref()
            // Make tuple with other option if it's some.
            .map(|membership| verified_role_id.map(|role_id| (membership, role_id)))
            // Flatten to Option<(...)>
            .flatten()
            // Filter to membership containing verified role.
            .filter(|(membership, role_id)| membership.roles.contains(role_id))
            .is_some();

        // Add verified status to template.
        template["discord"]["target"]["is_verified"] = json!(is_verified);

        // Add Discord authentication status to template.
        template["discord"]["viewer"]["is_authenticated"] = identity
            .identity()
            .await
            .map(|cookie| json!(cookie.get_discord().is_some()))
            .unwrap_or(json!(false));
    }

    // Render the profile template and send to user.
    return template.render_into_page(&req, page_title).await;
}

/// Create a form template for the user settings page.
fn make_settings_form() -> FormTemplate {
    // Create the base form.
    let mut form: FormTemplate = FormTemplate::new(SETTINGS_FORM, "Edit Profile");

    // The max entry year should always be the current year.
    form.template = json!({
        "max_entry_year": Local::today().year()
    });

    return form;
}

/// Get the viewer's user ID and make a profile edit form for them.
async fn get_context_and_make_form(
    auth: &AuthenticationCookie,
) -> Result<FormTemplate, TelescopeError> {
    // Get viewer's user ID. You have to be authenticated to edit your own profile.
    let viewer: Uuid = auth.get_user_id_or_error().await?;
    // Get the context for the edit form.
    let context = EditProfileContext::get(viewer).await?;
    // Ensure that the context exists.
    if context.is_none() {
        // Use an ISE since we should be able to get an edit context as long as there is an
        // authenticated user.
        return Err(TelescopeError::ise(format!(
            "Could not get edit context for user ID {}.",
            viewer
        )));
    }

    // Unwrap the context.
    let context = context.unwrap();

    // Create the form to edit the profile.
    let mut form: FormTemplate = make_settings_form();
    // Add the context to the form.
    form.template["context"] = json!(&context);
    // Add user id to the form for the cancel button
    form.template["user_id"] = json!(&viewer);

    // Add the list of roles (and whether the current role can switch to them).
    let role_list = UserRole::ALL_ROLES
        .iter()
        // Add availability data
        .map(|role| (*role, UserRole::can_switch_to(context.role, *role)))
        // Collect into map
        .collect::<HashMap<_, _>>();

    // Add to form.
    form.template["roles"] = json!(role_list);

    // Disable student role if the current role is external and there is no RCS ID in the context.
    if context.role.is_external() && context.rcs_id.first().is_none() {
        form.template["roles"]["student"] = json!(false);
    }

    return Ok(form);
}

/// User settings form.
#[get("/edit_profile")]
async fn settings(auth: AuthenticationCookie) -> Result<FormTemplate, TelescopeError> {
    return get_context_and_make_form(&auth).await;
}

/// Edits to the user's profile submitted through the form.
#[derive(Clone, Serialize, Deserialize, Debug)]
struct ProfileEdits {
    first_name: String,
    last_name: String,
    role: UserRole,

    /// Entry year for RPI students.
    #[serde(default)]
    cohort: String,
}

/// Submission endpoint for the user settings form.
#[post("/edit_profile")]
async fn save_changes(
    auth: AuthenticationCookie,
    Form(ProfileEdits {
        first_name,
        last_name,
        role,
        cohort,
    }): Form<ProfileEdits>,
) -> Result<HttpResponse, TelescopeError> {
    // Get authenticated user ID. This API call gets duplicated in the context creation unfortunately.
    let user_id = auth.get_user_id_or_error().await?;

    // Pass most of the handling here to the GET handler. This will get the context and make
    // and fill the form.
    let mut form: FormTemplate = get_context_and_make_form(&auth).await?;

    // Convert the cohort to a number or default to no cohort input. This should be checked client side.
    let cohort: Option<i64> = cohort.parse::<i64>().ok();

    // Check if user is allowed to set their cohort and if it is within the valid range.
    if cohort.is_some() {
        if auth.get_rcs_id().await.unwrap().is_none() {
            form.template["issues"]["cohort"] = json!("Please link RPI CAS before setting this.");
            return Err(TelescopeError::invalid_form(&form));
        }
        /*if form.template["roles"]["student"] == json!(false) {

        }*/
        let cohort_int = cohort.unwrap();
        let year: i64 = chrono::offset::Utc::now().year() as i64;
        if cohort_int < 1824 || cohort_int > year {
            form.template["issues"]["cohort"] = json!(
                format!("Year must be between 1824 and {}", year)
            );
            return Err(TelescopeError::invalid_form(&form));
        }
    }

    // Check that the current user role can switch to the submitted role.
    // First get the json version of the role as a string.
    let role_json: String = json!(role)
        .as_str()
        .expect("Role serialized to JSON string")
        .to_string();
    // Then index into the available roles on the context with the selected role to check availability.
    if form.template["roles"][&role_json] != json!(true) {
        return Err(TelescopeError::BadRequest {
            header: "Invalid Role Selection".into(),
            message: "The selected role is not available at this time".into(),
            show_status_code: false,
        });
    }

    // Fill the form with the submitted info.
    form.template["context"]["first_name"] = json!(&first_name);
    form.template["context"]["last_name"] = json!(&last_name);
    form.template["context"]["cohort"] = json!(&cohort);
    form.template["context"]["role"] = json!(role);

    // Error if first or last name is empty.
    if first_name.trim().is_empty() {
        form.template["issues"]["first_name"] = json!("Cannot be empty.");
        return Err(TelescopeError::invalid_form(&form));
    }

    if last_name.trim().is_empty() {
        form.template["issues"]["last_name"] = json!("Cannot be empty.");
        return Err(TelescopeError::invalid_form(&form));
    }

    // Execute GraphQL mutation to save changes.
    let user_id = SaveProfileEdits::execute(user_id, first_name, last_name, cohort, role)
        .await?
        .ok_or(TelescopeError::ise(
            "Could not save changes -- user not found.",
        ))?;

    // On success, redirect to user's profile.
    return Ok(HttpResponse::Found()
        .header(LOCATION, format!("/user/{}", user_id))
        .finish());
}
