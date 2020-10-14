use crate::web::{api::rest::login::LoginError, RequestContext, Template};

/// The Login Page.
#[derive(Clone, Debug, Serialize)]
pub struct LoginForm {
    redirect: String,
    email: Option<String>,
    error: Option<LoginError>,
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
            email: None,
            error: None,
        }
    }

    /// Add an error to the form.
    pub fn with_err(mut self, err: LoginError) -> Self {
        self.error = Some(err);
        self
    }

    /// Add a prefileld email to the form.
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
}

impl Template for LoginForm {
    const TEMPLATE_NAME: &'static str = "forms/login";
}
