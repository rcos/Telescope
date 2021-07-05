//! Services to support meeting edits.

use actix_web::web::{ServiceConfig, Path};
use crate::templates::Template;
use crate::error::TelescopeError;
use crate::web::services::auth::identity::AuthenticationCookie;
use crate::api::rcos::meetings::authorization_for::{UserMeetingAuthorization, AuthorizationFor};

/// Register the meeting edit services.
pub fn register(config: &mut ServiceConfig) {
    config.service(edit_page);
}

/// Service to display meeting edit form to users who can edit the meeting.
#[get("/meeting/{meeting_id}/edit")]
pub async fn edit_page(Path(meeting_id): Path<i64>, auth: AuthenticationCookie) -> Result<Template, TelescopeError> {
    // Check of the authenticated user can edit this meeting.
    let viewer: String = auth.get_rcos_username_or_error().await?;
    let authorization: UserMeetingAuthorization = AuthorizationFor::get(Some(viewer)).await?;

    // If the user cannot edit this meeting they are forbidden.
    if !authorization.can_edit_by_id(meeting_id).await? {
        return Err(TelescopeError::Forbidden);
    }

    Err(TelescopeError::NotImplemented)
}
