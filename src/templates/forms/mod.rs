//! Form templates, which support repeated unsuccessful submission until success
//! via a POST request and access but not submission via a GET request. Forms are
//! a special type of template commonly used in Telescope, and therefore have
//! their own traits.

use actix_web::{HttpRequest, HttpResponse};
use futures::future::LocalBoxFuture;
use crate::error::TelescopeError;
use serde::de::DeserializeOwned;
use actix_web::web::Form as ActixForm;
use std::future::Future;

pub mod common;

/// Trait for form templates. Specifies how forms behave when submitted
/// or accessed.
pub trait Form: Sized {
    /// The path to this forms handlebar template file from the
    /// templates directory.
    const TEMPLATE_PATH: &'static str;

    /// Create an empty instance of this form.
    fn empty() -> Self;

    /// The form data that is received and deserialized on this form's
    /// submission.
    type FormData: DeserializeOwned;

    /// Validate data submitted via this form. Returns `Ok(())` on success and
    /// an instance of this form with the validation errors on failure.
    fn validate(input: Self::FormData) -> Result<(), Self>;

    // /// The type of the future returned by the GET request handler.
    // type GetFut: Future<Output = Result<Self, TelescopeError>> + 'static;
    //
    // /// The type of the future returned by the POST request handler.
    // type PostFut: Future<Output = Result<HttpResponse, TelescopeError>> + 'static;
    //
    // /// How does this form respond to a GET request.
    // fn handle_get(req: HttpRequest) -> Self::GetFut;
    //
    // /// How does this form respond to a POST request with form data.
    // fn handle_post(req: HttpRequest, data: ActixForm<Self::FormData>) -> Self::PostFut;
}
