//! Registration form and constants.

use crate::templates::forms::Form;

/// The path from the templates directory to the registration template.
const TEMPLATE_PATH: &'static str = "forms/register";

/// Create registration form.
pub fn empty() -> Form {
    // Create form.
    let form = Form::new(TEMPLATE_PATH, "Create Account");

    // Create text fields
    let
}