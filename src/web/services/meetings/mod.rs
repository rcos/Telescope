//! Meetings page and services

use actix_web::HttpResponse;
use actix_web::web::{Path, ServiceConfig};
use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use crate::web::services::auth::identity::AuthenticationCookie;

mod list;
mod view;
mod create;

/// Register calendar related services.
pub fn register(config: &mut ServiceConfig) {
    list::register(config);

    config
        .service(edit_meeting)
        .service(delete_meeting)
        .service(submit_meeting_edit)
        .service(create::create_meeting)
        .service(create::submit_new_meeting)
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
