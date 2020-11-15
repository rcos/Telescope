use crate::web::Template;
use crate::models::recoveries::Recovery;

/// The email sent to users to recover their password.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordRecoveryEmail {
    /// The recovery record used to create this password recovery.
    recovery: Recovery,
    /// The link the user should follow to set a new password.
    link: String,
    /// The expiration of this recovery in UTC.
    expires_utc: String,
    /// The expiration of this recovery in Eastern time.
    expires_local: String,
}

/// The HTML formatted email sent to users trying to recover their password.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PasswordRecoveryEmailHtml {
    #[serde(flatten)]
    parent: PasswordRecoveryEmail
}

impl Template for PasswordRecoveryEmailHtml {
    const TEMPLATE_NAME: &'static str = "emails/recoveries/html";
}

/// The plaintext version of the email sent to users to recover their password.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PasswordRecoveryEmailText {
    #[serde(flatten)]
    parent: PasswordRecoveryEmail,
}

impl Template for PasswordRecoveryEmailText {
    const TEMPLATE_NAME: &'static str = "emails/recoveries/text";
}
