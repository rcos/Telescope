use crate::web::Template;

/// The registration page. This is a form that accepts a user's email,
/// checks if it's in the database already, and if not, emails the user a
/// confirmation link to let them set their password. (This also creates a
/// confirmation record in the database with an expiration time).
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RegistrationPage {
    /// The email the user last tried to register with.
    /// This is required for a success or error page.
    email: Option<String>,
    /// An error that occurred with registration.
    error: Option<String>,
    /// Show the success message indicating that a confirmation email was sent
    /// to the email specified in this object.
    success: bool,
}

impl RegistrationPage {
    /// Show the success message.
    pub fn success(email: impl Into<String>) -> Self {
        Self {
            success: true,
            email: Some(email.into()),
            error: None,
        }
    }

    /// Construct an error version of the form.
    ///
    /// This is a version that displays the email tried and an error message
    /// regarding the requested registration.
    pub fn error(email: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            email: Some(email.into()),
            error: Some(error.into()),
            success: false,
        }
    }
}

impl Template for RegistrationPage {
    const TEMPLATE_NAME: &'static str = "forms/register";
}
