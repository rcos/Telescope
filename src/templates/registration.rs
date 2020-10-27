use crate::web::Template;

/// The registration page. This is a form that accepts a user's email,
/// checks if it's in the database already, and if not, emails the user a
/// confirmation link to let them set their password. (This also creates a
/// confirmation record in the database with an expiration time).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistrationPage {

}

impl RegistrationPage {

}

impl Template for RegistrationPage {
    const TEMPLATE_NAME: &'static str = "forms/register";
}