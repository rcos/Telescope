//! Definition for the admin scope middleware. This authorizes all access to the admin pages.

use actix_web::dev::{Transform, Service, ServiceRequest, ServiceResponse};
use actix_web::Error as ActixError;
use std::future::Future;
use futures::prelude::future::Ready;
use futures::future::ok;
use std::task::{Context, Poll};
use std::pin::Pin;
use crate::error::TelescopeError;
use actix_identity::RequestIdentity;
use crate::web::services::auth::identity::AuthenticationCookie;
use crate::api::rcos::users::UserRole;
use crate::api::rcos::users::role_lookup::RoleLookup;
use std::rc::Rc;
use std::cell::RefCell;

/// Middleware to require admin authorization on requests.
pub struct AdminAuthorization;

/// A service with only admin access.
pub struct AdminOnly<S: 'static> {
    // Keep the inner service in an Rc RefCell so it can be passed to
    // the future without lifetime/ownership issues (or mutability issues)
    service: Rc<RefCell<S>>
}

impl<S> Transform<S> for AdminAuthorization
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = ActixError> + 'static,
    S::Future: 'static
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ActixError;
    type Transform = AdminOnly<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AdminOnly { service: Rc::new(RefCell::new(service)) })
    }
}

impl<S> Service for AdminOnly<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = ActixError> + 'static,
    S::Future: 'static
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
        let mut service = self.service.clone();

        // Box and pin the async value.
        return Box::pin(async move {
            // Verify the request originator is an admin.
            verify_is_admin(&req).await?;

            // Authorized! Go on to call the service.
            service.call(req).await
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
