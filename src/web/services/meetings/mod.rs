//! Meetings page and services

use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use crate::web::services::auth::identity::AuthenticationCookie;
use actix_web::web::{Path, ServiceConfig};
use actix_web::HttpResponse;

mod create;
mod list;
mod view;

/// Register calendar related services.
pub fn register(config: &mut ServiceConfig) {
    // Meetings list page
    list::register(config);

    // Meeting creation services
    create::register(config);

    config
        // Hide meeting mutations services until ready.
        // .service(edit_meeting)
        // .service(delete_meeting)
        // .service(submit_meeting_edit)

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
