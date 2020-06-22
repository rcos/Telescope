use crate::web::Template;

/// Login Button representative.
#[derive(Serialize)]
pub struct LoginButton;

impl Template for LoginButton {
    const TEMPLATE_NAME: &'static str = "static/navbar/login-button";
}
