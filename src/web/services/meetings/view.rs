//! Service to view a meeting's details.

use crate::api::rcos::meetings::authorization_for::{AuthorizationFor, UserMeetingAuthorization};
use crate::api::rcos::meetings::get_by_id::GetMeetingById;
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::tags::Tags;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use actix_web::web::Path;
use actix_web::HttpRequest;
use chrono::{Local, TimeZone};

/// The path from the templates directory to this template.
const TEMPLATE_PATH: &'static str = "meetings/page";

/// Endpoint to preview a specific meeting.
#[get("/meeting/{meeting_id}")]
pub async fn meeting(
    req: HttpRequest,
    Path(meeting_id): Path<i64>,
    identity: Identity,
) -> Result<Page, TelescopeError> {
    // Get the viewer's user ID.
    let viewer: Option<_> = identity.get_user_id().await?;
    // Get the viewer's authorization info.
    let authorization: UserMeetingAuthorization = AuthorizationFor::get(viewer).await?;
    // Get the meeting data from the RCOS API.
    let meeting: Option<_> = GetMeetingById::get(meeting_id).await?;
    // Check to make sure the meeting exists.
    if meeting.is_none() {
        return Err(TelescopeError::resource_not_found(
            "Meeting Not Found",
            "Could not find a meeting for this ID.",
        ));
    }

    // Unwrap the meeting object.
    let meeting = meeting.unwrap();
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
    // Set title and URL trivially.
    tags.title = meeting.title();
    tags.url = req.uri().to_string();

    // Build description.
    let mut description = String::new();
    let start = Local.from_utc_datetime(&meeting.start_date_time.naive_utc());
    let end = Local.from_utc_datetime(&meeting.end_date_time.naive_utc());
    if start.date() == end.date() {
        description.push_str(
            format!(
                "{} {} - {}",
                start.format("%B %-d, %Y"),
                start.format("%-I:%M %P"),
                end.format("%-I:%M %P")
            )
            .as_str(),
        );
    } else {
        description.push_str(
            format!(
                "{} at {} - {} at {}",
                start.format("%B %-d, %Y"),
                start.format("%-I:%M %P"),
                end.format("%B %-d, %Y"),
                end.format("%-I:%M %P")
            )
            .as_str(),
        );
    }

    // Add location if available.
    if meeting.location.is_some() {
        let location = meeting.location.as_ref().unwrap();
        if location != "" {
            description.push_str(format!(" @ {}", location).as_str());
        }
    } else if meeting.is_remote {
        description.push_str(" @ Remote");
    }

    // Add a newline for formatting.
    description.push_str("\n");

    // Add meeting type.
    description.push_str(format!("{}", meeting.type_).as_str());

    // Add the host if possible.
    if meeting.host.is_some() {
        let host = meeting.host.as_ref().unwrap();
        description
            .push_str(format!(" hosted by: {} {}", host.first_name, host.last_name).as_str());
    }

    description.push_str("\n");

    // Add meeting description.
    description.push_str(meeting.description.as_str());
    // Add description to OGP tags.
    tags.description = description;

    // Build meeting template.
    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({
        "meeting": &meeting,
        "auth": authorization
    });

    // Build page around meeting template.
    let mut page = template.in_page(&req, meeting.title()).await?;
    // Replace default page tags with meeting specific ones.
    page.ogp_tags = tags;
    // Return page.
    return Ok(page);
}
