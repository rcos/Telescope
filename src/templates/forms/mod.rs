//! Form templates, which support repeated unsuccessful submission until success
//! via a POST request and access but not submission via a GET request. Forms are
//! a special type of template commonly used in Telescope, and therefore have
//! their own traits.

use serde::Serialize;

pub mod common;

/// Trait for form fields. Each form field type should implement this trait
/// to define how to do validation.
pub trait FormField: Serialize {
    type Input;

    /// Check if a given input value would be valid for this field of the form.
    fn validate(&self, value: Self::Input) -> Option<String>;
}

#[derive(Serialize)]
pub struct Form {
    /// The handlebars file that should be used to render this form.
    #[serde(skip)]
    handlebars_file: &'static str,
}