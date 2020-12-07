use crate::{
    templates::{
        forms::common::text_field,
        Template
    },
    models::confirmations::Confirmation
};



/// The template for new account confirmations.
/// The user is prompted to input a name and password to seed their account.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUserConf {
    /// The confirmation that spawned this form.
    invite: Confirmation,
    /// The name previously entered into this form if there was one.
    pub name: Template,
    /// The user's new password.
    pub password: Template,
    /// The password again. Should match the other password field.
    pub confirm_password: Template,
}

impl NewUserConf {
    /// Template path from the templates directory.
    const TEMPLATE_NAME: &'static str = "forms/confirm/new_user";

    /// Create a new user confirmation template.
    pub fn new(conf: Confirmation) -> Self {
        Self {
            invite: conf,
            // Need to match the format of the form structure in
            // web/services/confirm.rs.
            name: text_field::plaintext_field("name", "Name")
                .field(text_field::PLACEHOLDER_FIELD, "Your Name"),
            password: text_field::password_field("password", "Password"),
            confirm_password: text_field::password_field("confirm", "Confirm Password"),
        }
    }
}

impl Into<Template> for NewUserConf {
    fn into(self) -> Template {
        Template::new(Self::TEMPLATE_NAME)
            .with_fields(self)
    }
}

/// An email confirmed for an existing user.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExistingUserConf {
    /// The invite that spawned this page.
    invite: Confirmation,
    /// An error message if an error occurred.
    error_message: Option<String>,
}

impl ExistingUserConf {
    /// Template path from template root.
    const TEMPLATE_NAME: &'static str = "forms/confirm/existing_user";

    /// Create a new existing user confirmation page.
    ///
    /// Panics if conf is not for an existing user.
    pub fn new(conf: Confirmation, err: Option<String>) -> Self {
        if conf.creates_user() {
            panic!("Cannot make ExistingUserConfirmation template for new user.")
        }
        Self {
            invite: conf,
            error_message: err,
        }
    }
}

impl Into<Template> for ExistingUserConf {
    fn into(self) -> Template {
        let mut t = Template::new(Self::TEMPLATE_NAME);
        t.append_fields(self);
        t
    }
}
