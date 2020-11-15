use crate::web::Template;
use crate::templates::forms::common::email::EmailField;

/// The password recovery form. This has a single field to indicate
/// the email to send the recovery link to. All fields of this struct are
/// therefore optional, as described below.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordRecoveryPage {
    /// A success message to indicate that the email was sent.
    success: Option<String>,
    /// The email form field.
    email_field: EmailField
}

impl PasswordRecoveryPage {
    /// Make a new recovery page.
    pub fn new() -> Self {
        Self {
            success: None,
            email_field: EmailField::new("email-input")
        }
    }

    /// Set the email field of this form.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Set the error field of this struct. This conflicts with the success
    /// field.
    pub fn error(mut self, err: impl Into<String>) -> Self {
        self.error = Some(err.into());
        self
    }

    /// Set the success field of this struct. This conflicts with the error
    /// field.
    pub fn success(mut self, msg: impl Into<String>) -> Self {
        self.success = Some(msg.into());
        self
    }
}

impl Template for PasswordRecoveryPage {
    const TEMPLATE_NAME: &'static str = "forms/forgot";
}
