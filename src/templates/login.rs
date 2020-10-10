use crate::web::{Template, RequestContext};

#[derive(Clone, Debug, Serialize)]
pub struct LoginForm {
    redirect: String,
}

impl LoginForm {
    /// The query variable that indicates what page the user is logging into.
    pub const REDIRECT_QUERY_VAR: &'static str = "to";

    /// Initialize a login form the redirect path from the query.
    pub fn from_context(ctx: &RequestContext) -> Self {
        let query = ctx.request().query_string();
        let mut parsed = url::form_urlencoded::parse(query.as_bytes());
        let target: String = parsed
            .find(|(k, _)| k == Self::REDIRECT_QUERY_VAR)
            .map(|(_, v)| v.into_owned())
            .unwrap_or("/".to_string());

        Self {
            redirect: target,
        }
    }
}

impl Template for LoginForm {
    const TEMPLATE_NAME: &'static str = "forms/login";
}