use crate::{models::recoveries::Recovery, templates::Template};
use chrono::Local;

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

impl PasswordRecoveryEmail {
    /// Template path for HTML template from template directory.
    const HTML_TEMPLATE: &'static str = "emails/recoveries/html";

    /// Template path for plaintext template from template directory.
    const TEXT_TEMPLATE: &'static str = "emails/recoveries/text";

    /// Construct a new password recovery email.
    pub fn new(recovery: Recovery, link: String) -> Self {
        let local_time = recovery.expiration.with_timezone(&Local).to_rfc2822();
        let utc_time = recovery.expiration.to_rfc2822();
        Self {
            recovery,
            link,
            expires_local: local_time,
            expires_utc: utc_time,
        }
    }

    /// Make a plaintext message from this.
    pub fn plaintext(&self) -> Template {
        Template::new(Self::TEXT_TEMPLATE).with_fields(self)
    }

    /// Make an html message from this.
    pub fn html(&self) -> Template {
        Template::new(Self::HTML_TEMPLATE).with_fields(self)
    }
}
