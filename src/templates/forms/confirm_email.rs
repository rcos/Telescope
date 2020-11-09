use crate::{
    models::Confirmation,
    web::Template
};

/// The template for email confirmations. On existing accounts, this is
/// just a success page. On new accounts, the user is prompted to input
/// a name and password.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailConfirmation {
    /// Was this email successfully added to an existing account?
    existing_success: bool,
    /// Was there an issue adding this email to an existing account.
    existing_fail: bool,
    /// The confirmation that spawned this form.
    invite: Confirmation,
    /// The name previously entered into this form.
    name: Option<String>,
    /// The error message if an error occurs.
    error_message: Option<String>
}

impl EmailConfirmation {
    /// Construct a confirmation page representing an email successfully confirmed for an existing
    /// account.
    pub fn existing_success(conf: Confirmation) -> Self {
        Self {
            existing_success: true,
            existing_fail: false,
            invite: conf,
            name: None,
            error_message: None,
        }
    }

    /// Construct an page reporting an error confirming an email for an existing account.
    pub fn existing_failure(conf: Confirmation, error: impl Into<String>) -> Self {
        Self {
            existing_success: false,
            existing_fail: true,
            invite: conf,
            name: None,
            error_message: Some(error.into())
        }
    }

    /// Construct the form to show to a user accepting an invite to create a
    /// new account.
    pub fn new_account(conf: Confirmation) -> Self {
        Self {
            existing_success: false,
            existing_fail: false,
            invite: conf,
            name: None,
            error_message: None
        }
    }

    pub fn new_account_error()

}

impl Template for EmailConfirmation {
    const TEMPLATE_NAME: &'static str = "forms/confirm";
}