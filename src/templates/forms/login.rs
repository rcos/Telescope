use crate::{
    templates::{
        forms::common::text_field,
        Template,
    },
    web::{
        api::rest::login::LoginError,
        RequestContext
    }
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

pub fn new()

/// The Login Page.
#[derive(Clone, Debug, Serialize)]
pub struct LoginForm {
    /// The page to redirect to.
    redirect: String,
    /// The email field in this form
    email: EmailField,
    /// The password field in this form.
    password: PasswordField,
}

impl LoginForm {
    /// The path to the template file from the templates directory.
    const TEMPLATE_NAME: &'static str = "forms/login";


    /// Get the URL of the page the user is attempting to access.
    pub fn target_page(ctx: &RequestContext) -> String {
        let query = ctx.request().query_string();
        let mut parsed = url::form_urlencoded::parse(query.as_bytes());
        parsed
            .find(|(k, _)| k == Self::REDIRECT_QUERY_VAR)
            .map(|(_, v)| v.into_owned())
            .unwrap_or("/".to_string())
    }

    /// Initialize a login form the redirect path from the query.
    pub fn from_context(ctx: &RequestContext) -> Self {
        Self {
            redirect: Self::target_page(ctx),
            email: EmailField::new("email"),
            password: PasswordField::new("password"),
        }
    }

    /// Add an error to the form.
    pub fn with_err(mut self, err: LoginError) -> Self {
        match err {
            LoginError::EmailNotFound => {
                self.email = self.email.error("Email not found.");
            }
            LoginError::WrongPassword => {
                self.password = self.password.error("Incorrect password.");
            }
        }
        self
    }

    /// Add a prefilled email to the form.
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = self.email.prefill(email);
        self
    }
}

impl Into<Template> for LoginForm {
    fn into(self) -> Template {
        Template::new(Self::TEMPLATE_NAME)
            .with_fields(self)
    }
}
