use crate::web::{api::rest::login::LoginError, RequestContext, Template};
use crate::templates::forms::common::email::EmailField;
use crate::templates::forms::common::password::PasswordField;

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
    /// The query variable that indicates what page the user is logging into.
    pub const REDIRECT_QUERY_VAR: &'static str = "to";

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
            },
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

impl Template for LoginForm {
    const TEMPLATE_NAME: &'static str = "forms/login";
}
