use crate::{
    models::confirmations::Confirmation,
    templates::{forms::common::text_field, Template},
};

/// Path to new user confirmation from templates directory. This form completes
/// the signup process after verifying an email.
const NEW_USER_CONF_FORM_TEMPLATE: &'static str = "forms/confirm/new_user";

/// Path to the form to display success (or error) message for existing user
/// who is confirming a new (additional) email.
const EXISTING_USER_CONF_TEMPLATE: &'static str = "forms/confirm/existing_user";

/// The serialized invite that spawned this confirmation page
/// (new users and existing users).
pub const INVITE: &'static str = "invite";

/// For new users, the handlebars field associated with the text field for
/// their name.
pub const NAME: &'static str = "name";

/// For new users, the handlebars field that is associated with the
/// password field.
/// This must match the structure of ['crate::web::services::NewUserConfInput'].
pub const PASSWORD: &'static str = "password";

/// For new users, the handlebars field that is associated with the
/// confirm password field.
/// This must match the structure of ['crate::web::services::NewUserConfInput'].
pub const CONFIRM: &'static str = "confirm";

/// For existing users, if there is an error confirming their email,
/// the handlebars field associated with that error message.
pub const ERROR: &'static str = "error";

/// Create the template for the confirmation page for a confirmation/invite
/// object.
pub fn for_conf(conf: &Confirmation) -> Template {
    if conf.creates_user() {
        // New User.
        Template::new(NEW_USER_CONF_FORM_TEMPLATE)
            .field(INVITE, conf)
            .field(NAME, text_field::plaintext_field(NAME, "Name"))
            .field(PASSWORD, text_field::password_field(PASSWORD, "Password"))
            .field(
                CONFIRM,
                text_field::password_field(CONFIRM, "Confirm Password"),
            )
    } else {
        // Existing user.
        Template::new(EXISTING_USER_CONF_TEMPLATE).field(INVITE, conf)
    }
}
