//! Definition for the admin scope middleware. This authorizes all access to the admin pages.

use crate::api::rcos::users::role_lookup::RoleLookup;
use crate::api::rcos::users::UserRole;
use crate::error::TelescopeError;
use crate::web::services::auth::identity::AuthenticationCookie;
use actix_identity::RequestIdentity;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error as ActixError;
use futures::future::ok;
use futures::prelude::future::Ready;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

/// Check that the identity associated with a service request is an admin.
async fn verify_is_admin(req: &ServiceRequest) -> Result<(), TelescopeError> {
    // First get the username of the authenticated user.
    let rcos_username = req
        // Get the identity of the service request -- this should be a json encoded authentication
        // cookie if it exists.
        .get_identity()
        // Deserialize the authentication cookie object if it exists.
        .and_then(|ident| serde_json::from_str::<AuthenticationCookie>(ident.as_str()).ok())
        // If not authenticated, return an error indicating so.
        .ok_or(TelescopeError::NotAuthenticated)?
        // Refresh the cookie if necessary.
        .refresh()
        .await?
        // Get the RCOS username associated with the authenticated user.
        .get_rcos_username()
        .await?
        // Respond with an error if the user is not found.
        .ok_or(TelescopeError::NotAuthenticated)?;

    // Then check that their role is admin.
    let role: UserRole = RoleLookup::get(rcos_username)
        .await?
        // The role should not be none, since the account needs to exist at this point.
        .expect("Viewer's account does not exist.");

    // Forbid access unless the user is an admin.
    if !role.is_admin() {
        Err(TelescopeError::Forbidden)
    } else {
        Ok(())
    }
}
