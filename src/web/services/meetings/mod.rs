//! Meetings page and services

use crate::api::rcos::meetings::authorization_for::{AuthorizationFor, UserMeetingAuthorization};
use crate::error::TelescopeError;
use crate::web::middlewares::authorization::Authorization;
use actix_web::web::ServiceConfig;
use uuid::Uuid;

mod create;
mod delete;
mod edit;
mod list;
mod view;

/// Register calendar related services.
pub fn register(config: &mut ServiceConfig) {
    // Meetings list page
    list::register(config);

    // Meeting creation services
    create::register(config);

    // Meeting edit services.
    edit::register(config);

    // Meeting destruction services.
    delete::register(config);

    config
        // The meeting viewing endpoint must be registered after the meeting creation endpoint,
        // so that the ID path doesn't match the create path.
        .service(view::meeting);
}

/// Create an authorization middleware based on a meeting authorization function.
fn make_meeting_auth_middleware<F: 'static + Fn(&UserMeetingAuthorization) -> bool>(
    f: &'static F,
) -> Authorization {
    Authorization::new(move |user_id: Uuid| {
        Box::pin(async move {
            // Get the user meeting access authorization object.
            let auth: UserMeetingAuthorization = AuthorizationFor::get(Some(user_id)).await?;

            // Call the verification function on the access authorization object.
            (f)(&auth).then(|| ()).ok_or(TelescopeError::Forbidden)
        })
    })
}
