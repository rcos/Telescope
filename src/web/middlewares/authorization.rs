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
use uuid::Uuid;

/// The type returned by authorization functions.
pub type AuthorizationResult = Result<(), TelescopeError>;

/// The type representing an authorization function reference.
/// Authorization functions accept an RCOS user ID and respond with
/// `Ok(())` on success or a telescope error preventing access.
type AuthorizationCheck = Rc<dyn Fn(Uuid) -> LocalBoxFuture<'static, AuthorizationResult>>;

/// Authorization middleware check's a user's credentials using a stored function
/// before calling the sub-service. This function may return any telescope error,
/// including [`TelescopeError::Forbidden`] to stop access to a resource.
///
/// This middleware is intended for use at the scope level.
#[derive(Clone)]
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
    pub fn new<F: 'static + Fn(Uuid) -> LocalBoxFuture<'static, AuthorizationResult>>(
        func: F,
    ) -> Self {
        Self {
            check: Rc::new(func),
        }
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
            check: self.check.clone(),
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
        let check: AuthorizationCheck = self.check.clone();

        // Box and pin the async value.
        return Box::pin(async move {
            // Extract the RCOS user ID.
            let user_id_result = extract_user_id(&req).await;

            // Properly propagate any errors.
            if let Err(error) = user_id_result {
                Ok(req.error_response(error))
            } else {
                let user_id = user_id_result.unwrap();

                // Call the authorization check.
                let authorization_result: AuthorizationResult = (check.as_ref())(user_id).await;

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

/// Extract the RCOS user ID authenticated with a request or error.
async fn extract_user_id(req: &ServiceRequest) -> Result<Uuid, TelescopeError> {
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
        // Get the RCOS user ID associated with the authenticated user.
        .get_user_id()
        .await?
        // Respond with an error if the user is not found.
        .ok_or(TelescopeError::NotAuthenticated)
}
