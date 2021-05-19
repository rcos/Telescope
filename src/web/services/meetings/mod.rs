//! Meetings page and services

use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::{Path, Query, ServiceConfig};
use chrono::{Date, DateTime, Duration, Local, NaiveDate, TimeZone, Utc};

use crate::api::rcos::meetings::{
    authorization_for::AuthorizationFor,
    get::{Meetings, meetings::MeetingsMeetings},
    get_by_id::{Meeting, meeting::MeetingMeeting},
    MeetingType,
};
use crate::api::rcos::meetings::authorization_for::UserMeetingAuthorization;
use crate::error::TelescopeError;
use crate::templates::{
    forms::{FormTemplate, meeting::create as creation_form},
    meetings, Template,
};
use crate::web::services::auth::identity::{AuthenticationCookie, Identity};

pub mod list_page;

/// Register calendar related services.
pub fn register(config: &mut ServiceConfig) {
    config
        .service(list_page::meetings_list)
        .service(edit_meeting)
        .service(delete_meeting)
        .service(submit_meeting_edit)
        .service(create_meeting)
        .service(submit_new_meeting)
        // The meeting viewing endpoint must be registered after the meeting creation endpoint,
        // so that the ID path doesn't match the create path.
        .service(meeting);
}

/// Endpoint to preview a specific meeting.
#[get("/meeting/{meeting_id}")]
async fn meeting(
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

    // If the meeting is visible to the viewer, make and return the template.
    return meetings::meeting_page::make(&meeting, &authorization)
        // Rendered inside a page
        .render_into_page(&req, meeting.title())
        // Wait for page to render and return result.
        .await;
}

/// Endpoint to edit a meeting.
#[get("/meeting/{meeting_id}/edit")]
async fn edit_meeting(
    Path(meeting_id): Path<i64>,
    auth: AuthenticationCookie,
) -> Result<FormTemplate, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}

/// Endpoint to delete a meeting.
#[get("/meeting/{meeting_id}/delete")]
async fn delete_meeting(
    Path(meeting_id): Path<i64>,
    auth: AuthenticationCookie,
) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}

/// Endpoint to submit a meeting edit.
#[post("/meeting/{meeting_id}/edit")]
async fn submit_meeting_edit(Path(meeting_id): Path<i64>) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}

/// Endpoint to create a meeting.
#[get("/meeting/create")]
async fn create_meeting(auth: AuthenticationCookie) -> Result<FormTemplate, TelescopeError> {
    // Check that the authenticated user has perms to create meetings.
    let username: String = auth.get_rcos_username_or_error().await?;
    let auth: UserMeetingAuthorization = AuthorizationFor::get(Some(username)).await?;
    if !auth.can_create_meetings() {
        return Err(TelescopeError::BadRequest {
            header: "Access Denied".into(),
            message: "The authenticated user does not have permission to create meetings. If you \
            think this is in error, please contact a coordinator."
                .into(),
            show_status_code: false,
        });
    }

    unimplemented!()
    // Otherwise return the meeting creation form.
    // return Ok(creation_form::make());
}

/// Endpoint to submit a meeting creation.
#[post("/meeting/create")]
async fn submit_new_meeting(auth: AuthenticationCookie) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}
