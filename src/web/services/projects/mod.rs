//! Services related to project management.

use actix_web::web::ServiceConfig;
use crate::web::middlewares::authorization::Authorization;
use crate::api::rcos::projects::authorization_for::{AuthorizationFor, UserProjectAuthorization};
use crate::error::TelescopeError;
use uuid::Uuid;

mod list;
mod view;
// mod create;

/// Register project services.
pub fn register(conf: &mut ServiceConfig) {
    conf.service(list::get);
    conf.service(view::project);
}

/// Create an authorization middleware based on a project authorization function.
pub fn make_projects_auth_middleware<F: 'static + Fn(&UserProjectAuthorization) -> bool>(
    f: &'static F,
) -> Authorization {
    Authorization::new(move |user_id: Uuid| {
        Box::pin(async move {
            // Get the user project access authorization object.
            let auth: UserProjectAuthorization = AuthorizationFor::get(Some(user_id)).await?;

            // Call the verification function on the access authorization object.
            (f)(&auth).then(|| ()).ok_or(TelescopeError::Forbidden)
        })
    })
}
//
