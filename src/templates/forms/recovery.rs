use crate::templates::{forms::common::text_field::{email_field, password_field, self}, Template};
use serde_json::Value;
use serde::Serialize;

/// The password recovery form. This has a single field to indicate
/// the email to send the recovery link to. All fields of this struct are
/// therefore optional, as described below.
#[derive(Debug, Clone, Serialize)]
pub struct ForgotPasswordPage {
    /// A success message to indicate that the email was sent.
    pub success: bool,
    /// The email form field.
    pub email_field: Template,
}

impl ForgotPasswordPage {
    /// The template path from the template root.
    const TEMPLATE_PATH: &'static str = "forms/recover/forgot";

    /// Make a new recovery page.
    pub fn new() -> Self {
        Self {
            success: false,
            email_field: email_field("email", "Email"),
        }
    }

    /// Set the email field of this form.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email_field.set_field(text_field::PREFILL_FIELD, email.into());
        self
    }

    /// Set the error field of this struct. This conflicts with the success
    /// field.
    pub fn error(mut self, err: impl Into<String>) -> Self {
        self.email_field.set_field(text_field::ERROR_FIELD, err.into());
        self
    }

    /// Convert self to a template via serialization.
    pub fn as_template(&self) -> Template {
        Template::new(Self::TEMPLATE_PATH).with_fields(self)
    }
}

/// Zero-sized type to namespace constants and functions for the template to set
/// a new password.
pub struct SetNewPassword;

impl SetNewPassword {
    /// Path to handlebars file.
    const PATH: &'static str = "forms/recovery/recover";

    /// Template key for new password text field.
    pub const NEW_PASS: &'static str = "new_pass";

    /// Template key for confirm password text field.
    pub const CONFIRM_PASS: &'static str = "confirm_pass";

    /// Create an empty form with fields to set and confirm the new password.
    pub fn empty() -> Template {
        let new_pass: Template = password_field(Self::NEW_PASS, "New Password");
        let confirm: Template = password_field(Self::CONFIRM_PASS, "Confirm New Password");

        Template::new(Self::PATH)
            .field(Self::NEW_PASS, new_pass)
            .field(Self::CONFIRM_PASS, confirm)
    }
}