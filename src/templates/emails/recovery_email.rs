use crate::web::Template;
use crate::models::recoveries::Recovery;
use chrono::FixedOffset;
use crate::models::emails::Email;

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
    /// Construct a new password recovery email.
    pub fn new(recovery: Recovery, link: String) -> Self {
        let local_offset = time::UtcOffset::current_local_offset().as_seconds();
        let time_zone = FixedOffset::east(local_offset);
        let local_time = recovery.expiration
            .with_timezone(&time_zone)
            .to_rfc2822();
        let utc_time = recovery.expiration.to_rfc2822();
        Self {
            recovery,
            link,
            expires_local: local_time,
            expires_utc: utc_time
        }
    }

    /// Make a plaintext message from this.
    pub fn plaintext(&self) -> PasswordRecoveryEmailText {
        PasswordRecoveryEmailText {
            parent: self.clone()
        }
    }

    /// Make an html message from this.
    pub fn html(&self) -> PasswordRecoveryEmailHtml {
        PasswordRecoveryEmailHtml {
            parent: self.clone()
        }
    }
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
