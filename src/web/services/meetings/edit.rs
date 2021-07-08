//! Services to support meeting edits.

use actix_web::web::{ServiceConfig, Path, Query};
use crate::templates::Template;
use crate::error::TelescopeError;
use crate::web::services::auth::identity::AuthenticationCookie;
use crate::api::rcos::meetings::authorization_for::{UserMeetingAuthorization, AuthorizationFor};
use crate::templates::forms::FormTemplate;
use crate::api::rcos::meetings::get_by_id::Meeting;
use chrono::{DateTime, Utc, Local};
use crate::api::rcos::meetings::ALL_MEETING_TYPES;

/// The Handlebars file for the meeting edit form.
const MEETING_EDIT_FORM: &'static str = "meetings/edit/form";

/// Register the meeting edit services.
pub fn register(config: &mut ServiceConfig) {
    config.service(edit_page);
}

/// Structure for query which can optionally be passed to the edit page to set a new host.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct HostQuery {
    /// The new host for the meeting. Empty string for no host.
    set_host: String
}

/// Service to display meeting edit form to users who can edit the meeting.
#[get("/meeting/{meeting_id}/edit")]
async fn edit_page(
    Path(meeting_id): Path<i64>,
    auth: AuthenticationCookie,
    set_host: Option<Query<HostQuery>>
) -> Result<FormTemplate, TelescopeError> {
    // Get the meeting data to check that it exists.
    let meeting_data = Meeting::get_by_id(meeting_id).await?;
    if meeting_data.is_none() {
        return Err(TelescopeError::resource_not_found("Meeting Not Found", "Could not find a meeting for this ID."));
    }
    // Unwrap option.
    let meeting_data = meeting_data.unwrap();

    // Check of the authenticated user can edit this meeting.
    let viewer: String = auth.get_rcos_username_or_error().await?;
    let authorization: UserMeetingAuthorization = AuthorizationFor::get(Some(viewer)).await?;

    // If the user cannot edit this meeting they are forbidden.
    let meeting_host: Option<&str> = meeting_data.host.as_ref().map(|host| host.username.as_str());
    if !authorization.can_edit(meeting_host) {
        return Err(TelescopeError::Forbidden);
    }

    // The user can edit this meeting. Use the meeting info to populate an edit form to send to them.
    let meeting_title: String = match meeting_data.title.as_ref() {
        Some(title) => title.clone(),
        None => {
            let meeting_start: &DateTime<Utc> = &meeting_data.start_date_time;
            format!("{} - {}", meeting_data.type_, meeting_start.date().naive_local().format("%B %_d, %Y"))
        }
    };

    // Create the form top show the user.
    let mut form: FormTemplate = FormTemplate::new(MEETING_EDIT_FORM, format!("Edit {}", meeting_title));

    form.template = json!({
        "data": &meeting_data,
        "meeting_types": ALL_MEETING_TYPES
    });

    // Add fields to the template converting the timestamps in the meeting data to the HTML versions.
    let meeting_start: &DateTime<Utc> = &meeting_data.start_date_time;
    form.template["data"]["start_date"] = json!(meeting_start.with_timezone(&Local).format("%Y-%m-%d").to_string());
    form.template["data"]["start_time"] = json!(meeting_start.with_timezone(&Local).format("%H:%M").to_string());

    let meeting_end: &DateTime<Utc> = &meeting_data.end_date_time;
    form.template["data"]["end_date"] = json!(meeting_end.naive_local().format("%Y-%m-%d").to_string());
    form.template["data"]["end_time"] = json!(meeting_end.naive_local().format("%H:%M").to_string());

    return Ok(form);
}
