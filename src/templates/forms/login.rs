use crate::{
    templates::{forms::common::text_field, Template},
    web::context::RequestContext,
};

/// The path to the template file from the template directory.
const TEMPLATE_PATH: &'static str = "forms/login";

/// Handlebars field for the location to redirect to after a successful login.
pub const REDIRECT: &'static str = "redirect";

/// The query variable that indicates what page the user is logging into.
pub const REDIRECT_QUERY_VAR: &'static str = "to";

/// Handlebars field for the email form component.
pub const EMAIL: &'static str = "email";

/// Handlebars field for the password form component.
pub const PASSWORD: &'static str = "password";

/// Get the URL of the page the user is attempting to access.
pub fn target_page(ctx: &RequestContext) -> String {
    let query = ctx.request().query_string();
    let mut parsed = url::form_urlencoded::parse(query.as_bytes());
    parsed
        .find(|(k, _)| k == REDIRECT_QUERY_VAR)
        .map(|(_, v)| v.into_owned())
        .unwrap_or("/".to_string())
}

/// Construct a new login form.
pub fn new(ctx: &RequestContext) -> Template {
    Template::new(TEMPLATE_PATH)
        .field(REDIRECT, target_page(ctx))
        .field(EMAIL, text_field::email_field(EMAIL, "Email"))
        .field(PASSWORD, text_field::password_field(PASSWORD, "Password"))
}
