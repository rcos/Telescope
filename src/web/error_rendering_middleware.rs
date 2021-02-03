//! Middleware for rendering telescope errors into full pages on the way out.

use actix_web::dev::{Transform, ServiceRequest, ServiceResponse, Service};
use actix_web::error::Error as ActixError;
use futures::future::{Ready, ok, BoxFuture};
use futures::task::{Context, Poll};
use std::pin::Pin;
use crate::error::TelescopeError;
use std::future::Future;

/// The factory to create handlers for telescope errors.
pub struct TelescopeErrorHandler;

/// Middleware to transform telescope errors in results from a service,
/// into the appropriate web pages.
pub struct TelescopeErrorHandlerMiddleware<S> {
    /// The next service in the chain.
    service: S
}

impl<S> Transform<S> for TelescopeErrorHandler
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = ActixError>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ActixError;
    type Transform = TelescopeErrorHandlerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(TelescopeErrorHandlerMiddleware { service })
    }
}

impl<S> Service for TelescopeErrorHandlerMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = ActixError>,
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
        debug!("Received: {:?}", req);
        // Extract the request path here to send to page renderer.
        let req_path: String = req.path().to_owned();

        // Call wrapped service.
        let service_response_future = self.service.call(req);

        // Create the pinned, boxed, async future here.
        Box::pin(async move {
            match service_response_future.await {
                // Pass through successful responses.
                Ok(s) => {
                    debug!("Sub-service success: {:?}", s);
                    Ok(s)
                },
                // Catch errors and check if they are telescope errors.
                Err(e) => {
                    debug!("Sub-service error: {:?}", e);
                    let actix_err: ActixError = e;
                    match actix_err.as_error::<TelescopeError>() {
                        Some(e) => {
                            debug!("Downgrade successful - Rendering error page");
                            // It is a telescope error! Render an error page.
                            let err: &TelescopeError = e;
                            err.render_error_page(req_path)
                        }
                        None => Err(actix_err)
                    }
                }
            }
        })
    }
}