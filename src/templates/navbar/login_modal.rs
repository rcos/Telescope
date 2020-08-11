use crate::web::Template;

/// Login Modal representative.
#[derive(Serialize)]
pub struct LoginModal;

impl Template for LoginModal {
    const TEMPLATE_NAME: &'static str = "static/navbar/login-modal";
}
