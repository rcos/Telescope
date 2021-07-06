//! Service to view a meeting's details.

use crate::api::rcos::meetings::authorization_for::{AuthorizationFor, UserMeetingAuthorization};
use crate::api::rcos::meetings::get_by_id::{meeting::MeetingMeeting, Meeting};
use crate::error::TelescopeError;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use actix_web::web::Path;
use actix_web::HttpRequest;

/// The path from the templates directory to this template.
const TEMPLATE_PATH: &'static str = "meetings/page";

/// Endpoint to preview a specific meeting.
#[get("/meeting/{meeting_id}")]
pub async fn meeting(
    req: HttpRequest,
    Path(meeting_id): Path<i64>,
    identity: Identity,
) -> Result<Template, TelescopeError> {
    // Get the viewer's username.
    let viewer_username: Option<String> = identity.get_rcos_username().await?;
    // Get the viewer's authorization info.
    let authorization: UserMeetingAuthorization = AuthorizationFor::get(viewer_username).await?;
    // Get the meeting data from the RCOS API.
    let meeting: Option<MeetingMeeting> = Meeting::get_by_id(meeting_id).await?;
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
    if meeting.is_draft && !authorization.can_view_drafts() {
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

    info!("{:#?}", &authorization);

    // If the meeting is visible to the viewer, make and return the template.
    return Template::new(TEMPLATE_PATH)
        .field("meeting", &meeting)
        .field("auth", authorization)
        // Rendered inside a page
        .render_into_page(&req, meeting.title())
        // Wait for page to render and return result.
        .await;
}
