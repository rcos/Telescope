use crate::templates::{
    forms::common::text_field,
    Template
};

/// The password recovery form. This has a single field to indicate
/// the email to send the recovery link to. All fields of this struct are
/// therefore optional, as described below.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            email_field: EmailField::new("email-input"),
        }
    }

    /// Set the email field of this form.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email_field = self.email_field.prefill(email);
        self
    }

    /// Set the error field of this struct. This conflicts with the success
    /// field.
    pub fn error(mut self, err: impl Into<String>) -> Self {
        self.email_field = self.email_field.error(err);
        self
    }
}

impl Into<Template> for PasswordRecoveryPage {
    fn into(self) -> Template {
        let mut t = Template::new(Self::TEMPLATE_NAME);
        t.append_fields(self);
        t
    }
}
