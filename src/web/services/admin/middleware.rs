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
use std::borrow::Borrow;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

/// Middleware to require admin authorization on requests.
pub struct AdminAuthorization;

/// A service with only admin access. Every call to a service wrapped in this
/// middleware queries the RCOS API twice
/// (once to get the username and once to check the user's role). Only users with an admin role
/// are authorized (all other requests error out before reaching the inner service).
pub struct AdminOnly<S: 'static> {
    // Keep the inner service in an Rc RefCell so it can be passed to
    // the future without lifetime/ownership issues (or mutability issues)
    service: Rc<RefCell<S>>,
}

impl<S> Transform<S> for AdminAuthorization
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ActixError;
    type Transform = AdminOnly<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AdminOnly {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

impl<S> Service for AdminOnly<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        // Clone a reference to the inner service, so that self is not referenced by the future.
        let mut service: Rc<RefCell<S>> = self.service.clone();

        // Box and pin the async value.
        return Box::pin(async move {
            // Verify the request originator is an admin.
            let verification_result = verify_is_admin(&req).await;

            // Check for an error. We have to explicitly convert to a response here otherwise
            // actix error handling will skip upstream middlewares.
            if let Err(telescope_error) = verification_result {
                Ok(req.error_response(telescope_error))
            } else {
                // Otherwise, we are authorized! Go on to call the service.
                service.call(req).await
            }
        });
    }
}

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
