//! Meetings page and services

use actix_web::web::{Path, ServiceConfig};
use actix_web::{HttpRequest, HttpResponse};

use crate::api::rcos::meetings::authorization_for::AuthorizationFor;
use crate::api::rcos::meetings::authorization_for::UserMeetingAuthorization;
use crate::error::TelescopeError;
use crate::templates::{forms::FormTemplate, Template};
use crate::web::services::auth::identity::{AuthenticationCookie, Identity};

mod list;
mod view;

/// Register calendar related services.
pub fn register(config: &mut ServiceConfig) {
    list::register(config);

    config
        .service(edit_meeting)
        .service(delete_meeting)
        .service(submit_meeting_edit)
        .service(create_meeting)
        .service(submit_new_meeting)
        // The meeting viewing endpoint must be registered after the meeting creation endpoint,
        // so that the ID path doesn't match the create path.
        .service(view::meeting);
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
