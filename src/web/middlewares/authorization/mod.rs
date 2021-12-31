//! Middleware for resource access management (authorization).

pub mod util;

use crate::error::TelescopeError;
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
/// Authorization functions accept a reference to a `ServiceRequest` and use
/// it to determine whether the user can access a given page or endpoint. The
/// response should be `Ok(())` on success or a telescope error preventing access.
type AuthorizationCheck =
    Rc<dyn for<'a> Fn(&'a ServiceRequest) -> LocalBoxFuture<'a, AuthorizationResult>>;
// Use the higher rank lifetime bound to satisfy the compiler.

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
    pub fn new<F>(func: F) -> Self
    where
        F: 'static + for<'a> Fn(&'a ServiceRequest) -> LocalBoxFuture<'a, AuthorizationResult>,
    {
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
            // Call the authorization check.
            match (check.as_ref())(&req).await {
                // On error return a response with the rendered error.
                // This has to be an Ok variant, otherwise the actix system will bypass
                // other middlewares.
                Err(error) => Ok(req.error_response(error)),

                // Otherwise the user is authorized. Call the service.
                Ok(_) => service.call(req).await,
            }
        });
    }
}
