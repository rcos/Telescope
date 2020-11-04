use crate::web::Template;

/// The password recovery form. This has a single field to indicate
/// the email to send the recovery link to. All fields of this struct are
/// therefore optional, as described below.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PasswordRecoveryPage {
    /// The email previously used to submit this form.
    email: Option<String>,
    /// An error message, if the email was not found.
    error: Option<String>,
    /// A success message to indicate that the email was sent.
    success: Option<String>,
}

impl PasswordRecoveryPage {
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
