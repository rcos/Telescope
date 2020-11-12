use crate::{
    models::Confirmation,
    web::Template
};
use crate::templates::forms::common::password::PasswordField;

/// The template for new account confirmations.
/// The user is prompted to input a name and password to seed their account.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUserConfirmation {
    /// The confirmation that spawned this form.
    invite: Confirmation,
    /// The name previously entered into this form if there was one.
    name: Option<String>,
    /// The user's new password.
    password: PasswordField,
    /// The password again. Should match the other password field.
    confirm_password: PasswordField,
}

impl NewUserConfirmation {
    /// Create a new user confirmation template.
    pub fn new(conf: Confirmation) -> Self {
        Self {
            invite: conf,
            name: None,
            // these last two need to match the format of the form structure in
            // web/services/confirm.rs
            password: PasswordField::new("password"),
            confirm_password: PasswordField::new("confirm-password")
                .map_common(|c| c.name("confirm"))
        }
    }
}

impl Template for NewUserConfirmation {
    const TEMPLATE_NAME: &'static str = "forms/confirm/new_user";
}

/// An email confirmed for an existing user.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExistingUserConfirmation {
    /// The invite that spawned this page.
    invite: Confirmation,
    /// An error message if an error occurred.
    error_message: Option<String>,
}

impl Template for ExistingUserConfirmation {
    const TEMPLATE_NAME: &'static str = "forms/confirm/existing_user";
}
