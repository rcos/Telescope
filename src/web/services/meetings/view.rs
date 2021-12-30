//! Service to view a meeting's details.

use crate::api::rcos::meetings::authorization_for::{AuthorizationFor, UserMeetingAuthorization};
use crate::api::rcos::meetings::get_by_id::{meeting::MeetingMeeting, Meeting};
use crate::error::TelescopeError;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use actix_web::web::Path;
use actix_web::HttpRequest;
use chrono::{Local, TimeZone};
use crate::env::global_config;
use crate::templates::tags::Tags;

/// The path from the templates directory to this template.
const TEMPLATE_PATH: &'static str = "meetings/page";

/// Endpoint to preview a specific meeting.
#[get("/meeting/{meeting_id}")]
pub async fn meeting(
    req: HttpRequest,
    Path(meeting_id): Path<i64>,
    identity: Identity,
) -> Result<Template, TelescopeError> {
    // Get the viewer's user ID.
    let viewer: Option<_> = identity.get_user_id().await?;
    // Get the viewer's authorization info.
    let authorization: UserMeetingAuthorization = AuthorizationFor::get(viewer).await?;
    // Get the meeting data from the RCOS API.
    let meeting: Option<MeetingMeeting> = Meeting::get(meeting_id).await?;
    // Check to make sure the meeting exists.
    if meeting.is_none() {
        return Err(TelescopeError::resource_not_found(
            "Meeting Not Found",
            "Could not find a meeting for this ID.",
        ));
    }

    // Unwrap the meeting object.
    let meeting: MeetingMeeting = meeting.unwrap();
    // Make sure that the meeting is visible to the user.
    // First check for draft status.
    let meeting_host: Option<_> = meeting.host.as_ref().map(|host| host.id);
    let can_edit: bool = authorization.can_edit(meeting_host);
    if !can_edit && meeting.is_draft && !authorization.can_view_drafts() {
        return Err(TelescopeError::BadRequest {
            header: "Meeting Not Visible".into(),
            message: "This meeting is currently marked as a draft and is only visible to \
            coordinators and faculty advisors. If you believe this is in error, please \
            contact a coordinator."
                .into(),
            show_status_code: false,
        });
    }

    // Then check the meeting variant.
    if !authorization.can_view(meeting.type_) {
        return Err(TelescopeError::BadRequest {
            header: "Meeting Access Restricted".into(),
            message: "Access to this meeting is restricted to mentors or coordinators. If you \
            think this is in error, please contact a coordinator."
                .into(),
            show_status_code: false,
        });
    }

    // Create dynamic OGP tags and start with default so all other fields are correct
    let mut tags = Tags::default();
    tags.title = if meeting.title.is_some() { format!("RCOS {} - {}", meeting.type_, meeting.title()) } else { meeting.title() };
    tags.url = format!("{}/meeting/{}", global_config().discord_config.telescope_url, meeting_id);

    let mut description = String::new();
    let start = Local.from_utc_datetime(&meeting.start_date_time.naive_utc());
    let end = Local.from_utc_datetime(&meeting.end_date_time.naive_utc());
    if start.date() == end.date() {
        description.push_str(format!("{}{} -{}", start.format("%B %_d, %Y"), start.format("%_I:%M %P"), end.format("%_I:%M %P")).as_str());
    }
    else {
        description.push_str(format!("{} at{} - {} at{}", start.format("%B %_d, %Y"), start.format("%_I:%M %P"),
                                     end.format("%B %_d, %Y"), end.format("%_I:%M %P")).as_str());
    }
    if meeting.location.is_some() {
        let location = meeting.location.as_ref().unwrap();
        if location != "" {
            description.push_str(format!(" @ {}", location).as_str());
        }
    }
    else if meeting.is_remote {
        description.push_str(" @ Remote");
    }
    description.push_str("\n");
    if meeting.host.is_some() {
        let host = meeting.host.as_ref().unwrap();
        description.push_str(format!("Hosted By: {} {}\n", host.first_name, host.last_name).as_str());
    }
    if meeting.description != "" {
        description.push_str(meeting.description.as_str());
    }
    tags.description = description;

    // If the meeting is visible to the viewer, make and return the template.
    return Template::new(TEMPLATE_PATH)
        .field("meeting", &meeting)
        .field("auth", authorization)
        // Rendered inside a page
        .render_into_page_with_tags(&req, meeting.title(), Some(tags))
        // Wait for page to render and return result.
        .await;
}
