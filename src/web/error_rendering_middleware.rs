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
use futures::TryStreamExt;
use actix_web::web::Buf;
use actix_web::HttpRequest;

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
            // Wait for the service response to resolve. Propagate any
            // errors that have not already been converted to an HTTP
            // response. (All telescope errors should have been serialized
            // into an HTTP response at this point).
            let mut service_response: ServiceResponse = service_response_future.await?;


            // See if the success response is a serialized telescope error.
            let has_telescope_mime: bool = service_response.headers()
                .get(CONTENT_TYPE)
                .map_or(false, |val| {
                    val == TELESCOPE_ERROR_MIME
                });

            // If not just return it as is.
            if !has_telescope_mime {
                return Ok(service_response);
            }

            // If it is, we will collect the body and deserialize the error
            // from it.
            // First, get the body without destroying or loosing ownership of the service response.
            // This will remove the body from the response, leaving the response with no body.
            let body: ResponseBody<Body> = service_response.response_mut().take_body();
            // Then convert it to a string using Stream future utility functions.
            let body_str: String = body
                // Convert every segment of the body into a string.
                .map_ok(|bytes| String::from_utf8_lossy(bytes.as_ref()).to_string())
                // Collect all of the segments of the stream into one string.
                .try_collect::<String>()
                // Waif for the stream to collect and propagate any errors.
                .await?;

            // Deserialize the telescope error from the response.
            let err: TelescopeError = serde_json::from_str(body_str.as_str())
                // Convert and propagate any serialization errors.
                .map_err(ActixError::from)?;

            // Get a reference to the original request.
            let req: &HttpRequest = service_response.request();
            // Render the error page to a string
            let rendered: String = err.render_error_page(req)?;
            // Convert the rendered page into a response.
            let intermediate_response: HttpResponse = rendered.into();
            // Return the original service response with the body from the
            // rendered error response.
            let final_response: ServiceResponse = service_response.into_response(intermediate_response);
            return Ok(final_response);
        })
    }
}