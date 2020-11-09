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

}

impl Template for EmailConfirmation {
    const TEMPLATE_NAME: &'static str = "forms/confirm";
}