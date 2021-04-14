//! Meetings page and services

use crate::templates::{
    Template,
    forms::Form,
    meetings
};
use crate::error::TelescopeError;
use actix_web::web::{ServiceConfig, Query, Path};
use actix_web::{HttpRequest, HttpResponse};
use chrono::{DateTime, Utc, TimeZone, Local, Duration, NaiveDate, Date};
use crate::web::services::auth::identity::{Identity, AuthenticationCookie};
use crate::web::api::rcos::meetings::{get::{
    Meetings,
    meetings::MeetingsMeetings
}, get_by_id::{
    Meeting,
    meeting::MeetingMeeting,
}, authorization_for::AuthorizationFor, MeetingType};
use crate::web::api::rcos::meetings::authorization_for::UserMeetingAuthorization;

/// Register calendar related services.
pub fn register(config: &mut ServiceConfig) {
    config
        .service(meetings_list)
        .service(edit_meeting)
        .service(delete_meeting)
        .service(submit_meeting_edit)
        .service(create_meeting)
        .service(submit_new_meeting)
        // The meeting viewing endpoint must be registered after the meeting creation endpoint,
        // so that the ID path doesn't match the create path.
        .service(meeting);
}

/// Query parameters submitted via the form on the meetings page.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct MeetingsQuery {
    /// The start time to get events from.
    pub start: NaiveDate,
    /// The end time to get events from.
    pub end: NaiveDate,
}

/// Meetings page
#[get("/meetings")]
async fn meetings_list(req: HttpRequest, params: Option<Query<MeetingsQuery>>, identity: Identity) -> Result<Template, TelescopeError> {
    // Resolve parameters to API query variables
    let start: DateTime<Utc> = params.as_ref()
        // Extract the start parameter from the query
        .map(|p| p.start)
        // Convert to a date in the local timezone
        .map(|naive: NaiveDate| Local.from_local_date(&naive))
        // If it's ambiguous what date to use in the local timezone, pick the earlier one.
        .and_then(|local_result| local_result.earliest())
        // Conver the date to a timestamp of the beginning of the day
        .map(|date: Date<Local>| date.and_hms(0,0,0))
        // If there is no valid timezone or the start parameter wasn't supplied,
        // use the current time minus 2 hours. This should be sufficient to catch all
        // recent and ongoing meetings.
        .unwrap_or(Local::now() - Duration::hours(2))
        // Convert timezone to UTC.
        .with_timezone(&Utc);

    let end: DateTime<Utc> = params.as_ref()
        // Extract the end parameter from the query
        .map(|p| p.end)
        // Convert to a date in the local timezone.
        .map(|naive: NaiveDate| Local.from_local_date(&naive))
        // If the date in the local timezone is ambiguous, use the later one
        .and_then(|local_result| local_result.latest())
        // Convert the date to a timestamp near midnight that night.
        .map(|date: Date<Local>| date.and_hms(23,59,59))
        // If there is no valid time, or the parameter wasn't supplied,
        // default to one week from today. This will show all the next meetings.
        .unwrap_or(Local::now() + Duration::weeks(1))
        // Convert timezone to UTC.
        .with_timezone(&Utc);

    // Is there an RCOS user authenticated?
    let viewer_username: Option<String> = identity.get_rcos_username().await?;
    // Check if that user can view drafts / certain meeting types.
    let authorization: UserMeetingAuthorization = AuthorizationFor::get(viewer_username).await?;
    let include_drafts: bool = authorization.can_view_drafts();
    let visible_meeting_types: Vec<MeetingType> = authorization.viewable_types();

    // Query the RCOS API to get meeting data.
    let events: Vec<MeetingsMeetings> =
        Meetings::get(start, end, include_drafts, visible_meeting_types).await?;

    // Get the values to pre-fill in the filters.
    let query = params
        // The existing query if there was one
        .map(|p| p.0)
        // Otherwise convert the API parameters
        .unwrap_or(MeetingsQuery {
            start: start.naive_local().date(),
            end: end.naive_local().date(),
        });

    // Build a meetings page template, render it into a page for the user.
    return meetings::list_page::make(events, Some(query), &authorization)
        .render_into_page(&req, "RCOS Meetings")
        .await;
}

/// Endpoint to preview a specific meeting.
#[get("/meeting/{meeting_id}")]
async fn meeting(req: HttpRequest, Path(meeting_id): Path<i64>, identity: Identity) -> Result<Template, TelescopeError> {
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
            "Could not find a meeting for this ID."
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
            contact a coordinator.".into(),
            show_status_code: false
        });
    }

    // Then check the meeting variant.
    if !authorization.can_view(meeting.type_) {
        return Err(TelescopeError::BadRequest {
            header: "Meeting Access Restricted".into(),
            message: "Access to this meeting is restricted to mentors or coordinators. If you \
            think this is in error, please contact a coordinator.".into(),
            show_status_code: false
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
async fn edit_meeting(Path(meeting_id): Path<i64>, auth: AuthenticationCookie) -> Result<Form, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}

/// Endpoint to delete a meeting.
#[get("/meeting/{meeting_id}/delete")]
async fn delete_meeting(Path(meeting_id): Path<i64>, auth: AuthenticationCookie) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}

/// Endpoint to submit a meeting edit.
#[post("/meeting/{meeting_id}/edit")]
async fn submit_meeting_edit(Path(meeting_id): Path<i64>) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}

/// Endpoint to create a meeting.
#[get("/meeting/create")]
async fn create_meeting(auth: AuthenticationCookie) -> Result<Form, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}

/// Endpoint to submit a meeting creation.
#[post("/meeting/create")]
async fn submit_new_meeting(auth: AuthenticationCookie) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}
