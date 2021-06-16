//! Middleware for resource access management (authorization).

use crate::error::TelescopeError;
use std::rc::Rc;
use std::cell::RefCell;
use actix_web::{
    error::Error as ActixError,
    dev::{Service, Transform, ServiceRequest, ServiceResponse}
};
use futures::future::{Ready, ok, LocalBoxFuture};
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context};

/// The type returned by authorization functions.
pub type AuthorizationResult = Result<(), TelescopeError>;

/// The type representing an authorization function reference.
pub type AuthorizationCheck = fn(&ServiceRequest) -> LocalBoxFuture<AuthorizationResult>;

/// Authorization middleware check's a user's credentials using a stored function
/// before calling the sub-service. This function may return any telescope error,
/// including [`TelescopeError::Forbidden`] to stop access to a resource.
///
/// This middleware is intended for use at the scope level.
pub struct Authorization {
    /// The function to check authorization before calling the service.
    check: AuthorizationCheck
}

/// Wrapper type that provides authorization gated access to a service.
pub struct AuthorizedAccess<S: 'static> {
    /// The function to check authorization before calling the service.
    check: AuthorizationCheck,

    /// The service. This is stored in an [`Rc`]'d [`RefCell`] to allow the
    /// service response future to keep a cloned reference to the service after
    /// the transform exits.
    service: Rc<RefCell<S>>
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
            check: self.check
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
            // Call the authorization check.
            let authorization_result: AuthorizationResult = check(&req).await;

            // Check for an error. We have to explicitly convert to a response here otherwise
            // actix error handling will skip upstream middlewares.
            if let Err(telescope_error) = authorization_result {
                Ok(req.error_response(telescope_error))
            } else {
                // Otherwise, we are authorized! Go on to call the service.
                service.call(req).await
            }
        });
    }
}
