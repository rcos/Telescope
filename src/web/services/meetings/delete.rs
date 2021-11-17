//! Services for deleting meetings.

use crate::api::rcos::meetings::authorization_for::{AuthorizationFor, UserMeetingAuthorization};
use crate::api::rcos::meetings::delete::DeleteMeeting;
use crate::error::TelescopeError;
use crate::web::services::auth::identity::AuthenticationCookie;
use actix_web::http::header::LOCATION;
use actix_web::web::{Path, ServiceConfig};
use actix_web::HttpResponse;

/// Register meeting deletion services.
pub fn register(config: &mut ServiceConfig) {
    config.service(delete_meeting);
}

/// Meeting deletion endpoint. Uses post to prevent inadvertent deletion.
#[post("/meeting/{meeting_id}/delete")]
async fn delete_meeting(
    auth: AuthenticationCookie,
    Path(meeting_id): Path<i64>,
) -> Result<HttpResponse, TelescopeError> {
    // Require that there is a user authenticated.
    let user_id = auth.get_user_id_or_error().await?;
    // Require that they can delete meetings.
    let auth: UserMeetingAuthorization = AuthorizationFor::get(Some(user_id)).await?;
    if !auth.can_delete_meetings() {
        return Err(TelescopeError::Forbidden);
    }

    // Authorized. Delete the meeting and associated attendances.
    let api_response = DeleteMeeting::execute(meeting_id).await?;
    // Check that there was a meeting delete.
    if api_response.delete_meetings_by_pk.is_none() {
        return Err(TelescopeError::ise(
            "Meeting Deletion did not return meeting ID.",
        ));
    }

    // Meeting deleted successfully. Redirect user back to meetings page.
    Ok(HttpResponse::Found().header(LOCATION, "/meetings").finish())
}
