//! Middleware for resource access management (authorization).

use crate::error::TelescopeError;
use crate::web::services::auth::identity::AuthenticationCookie;
use actix_identity::RequestIdentity;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::Error as ActixError,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

/// The type returned by authorization functions.
pub type AuthorizationResult = Result<(), TelescopeError>;

/// The type representing an authorization function reference.
/// Authorization functions accept an RCOS username and respond with
/// `Ok(())` on success or a telescope error preventing access.
pub type AuthorizationCheck = fn(String) -> LocalBoxFuture<'static, AuthorizationResult>;

/// Authorization middleware check's a user's credentials using a stored function
/// before calling the sub-service. This function may return any telescope error,
/// including [`TelescopeError::Forbidden`] to stop access to a resource.
///
/// This middleware is intended for use at the scope level.
#[derive(Copy, Clone)]
pub struct Authorization {
    /// The function to check authorization before calling the service.
    check: AuthorizationCheck,
}

/// Wrapper type that provides authorization gated access to a service.
pub struct AuthorizedAccess<S: 'static> {
    /// The function to check authorization before calling the service.
    check: AuthorizationCheck,

    /// The service. This is stored in an [`Rc`]'d [`RefCell`] to allow the
    /// service response future to keep a cloned reference to the service after
    /// the transform exits.
    service: Rc<RefCell<S>>,
}

impl Authorization {
    /// Construct a new authorization transform.
    pub fn new(func: AuthorizationCheck) -> Self {
        Self { check: func }
    }
}

impl<S> Transform<S> for Authorization
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ActixError;
    type Transform = AuthorizedAccess<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthorizedAccess {
            service: Rc::new(RefCell::new(service)),
            check: self.check,
        })
    }
}

impl<S> Service for AuthorizedAccess<S>
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
        // Copy the function pointer to the authorization check to avoid moving self.
        let check: AuthorizationCheck = self.check;

        // Box and pin the async value.
        return Box::pin(async move {
            // Extract the RCOS username.
            let rcos_username = extract_rcos_username(&req).await;

            // Properly propagate any errors.
            if let Err(error) = rcos_username {
                Ok(req.error_response(error))
            } else {
                let rcos_username = rcos_username.unwrap();

                // Call the authorization check.
                let authorization_result: AuthorizationResult = (check)(rcos_username).await;

                // Check for an error. We have to explicitly convert to a response here otherwise
                // actix error handling will skip upstream middlewares.
                if let Err(telescope_error) = authorization_result {
                    Ok(req.error_response(telescope_error))
                } else {
                    // Otherwise, we are authorized! Go on to call the service.
                    service.call(req).await
                }
            }
        });
    }
}

/// Extract the RCOS username authenticated with a request or error.
async fn extract_rcos_username(req: &ServiceRequest) -> Result<String, TelescopeError> {
    req
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
        .ok_or(TelescopeError::NotAuthenticated)
}
