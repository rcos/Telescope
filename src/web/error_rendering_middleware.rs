//! Middleware for rendering telescope errors into full pages on the way out.

use actix_web::dev::{Transform, ServiceRequest, ServiceResponse, Service};
use actix_web::error::Error as ActixError;
use futures::future::{Ready, ok, BoxFuture};
use futures::task::{Context, Poll};
use std::pin::Pin;
use crate::error::{TelescopeError, TELESCOPE_ERROR_MIME};
use std::future::Future;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::HttpResponse;
use actix_web::body::{ResponseBody, Body};
use futures::prelude::*;
use futures::TryStreamExt;

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
        // Call wrapped service.
        let service_response_future = self.service.call(req);

        // Create the pinned, boxed, async future here.
        Box::pin(async move {
            match service_response_future.await {
                Ok(s) => {
                    // See if the success response is a serialized telescope
                    // error.
                    let mut service_response: ServiceResponse = s;
                    let has_telescope_mime: bool = service_response.headers()
                        .get(CONTENT_TYPE)
                        .map_or(false, |val| {
                            val == TELESCOPE_ERROR_MIME
                        });

                    if has_telescope_mime {
                        // If we have the custom telescope MIME type,
                        // get the response's body. This type is a Stream
                        // future.
                        let response_body: ResponseBody<Body> = service_response.take_body()
                            // Map all the actix errors into futures
                            .try_concat()

                    } else {
                        // If it doesn't have the custom telescope MIME type,
                        // pass it on to the next middleware or the user.
                        Ok(service_response)
                    }
                },

                // Propagate any errors that have not been converted to
                // a successful response with JSON in the right MIME type.
                error => error
            }
        })
    }
}