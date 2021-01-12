use crate::templates::{forms::common::text_field, Template};
use serde_json::Value;

/// The password recovery form. This has a single field to indicate
/// the email to send the recovery link to. All fields of this struct are
/// therefore optional, as described below.
#[derive(Debug, Clone, Serialize)]
pub struct PasswordRecoveryPage {
    /// A success message to indicate that the email was sent.
    pub success: bool,
    /// The email form field.
    pub email_field: Template,
}

impl PasswordRecoveryPage {
    /// The template path from the template root.
    const TEMPLATE_NAME: &'static str = "forms/forgot";

    /// Make a new recovery page.
    pub fn new() -> Self {
        Self {
            success: false,
            email_field: text_field::email_field("email", "Email"),
        }
    }

    /// Set the email field of this form.
    pub fn email(mut self, email: impl Into<Value>) -> Self {
        self.email_field[text_field::PREFILL_FIELD] = email.into();
        self
    }

    /// Set the error field of this struct. This conflicts with the success
    /// field.
    pub fn error(mut self, err: impl Into<Value>) -> Self {
        self.email_field[text_field::ERROR_FIELD] = err.into();
        self
    }

    /// Convert self to a template via serialization.
    pub fn as_template(&self) -> Template {
        Template::new(Self::TEMPLATE_NAME).with_fields(self)
    }
}
