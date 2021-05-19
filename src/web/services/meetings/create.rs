//! Creation form and services for meetings.

use crate::api::rcos::meetings::authorization_for::{AuthorizationFor, UserMeetingAuthorization};
use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use crate::web::services::auth::identity::AuthenticationCookie;
use actix_web::HttpResponse;

/// The path to the handlebars template to create a meeting.
const TEMPLATE_PATH: &'static str = "forms/meeting/create";

/// Endpoint to create a meeting.
#[get("/meeting/create")]
pub async fn create_meeting(auth: AuthenticationCookie) -> Result<FormTemplate, TelescopeError> {
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
pub async fn submit_new_meeting(
    auth: AuthenticationCookie,
) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}
