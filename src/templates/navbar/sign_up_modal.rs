use crate::web::Template;

/// Sign-up Modal representative.
#[derive(Serialize)]
pub struct SignUpModal;

impl Template for SignUpModal {
    const TEMPLATE_NAME: &'static str = "static/navbar/sign-up-modal";
}
