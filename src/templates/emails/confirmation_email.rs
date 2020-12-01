use crate::{
    models::confirmations::Confirmation,
    web::RequestContext,
    templates::Template
};
use chrono::FixedOffset;
use lettre_email::EmailBuilder;
use uuid::Uuid;

/// The HTML version of the email sent to new users asking them
/// to go to the telescope website and set their name and create a password.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConfirmationEmailHtml {
    /// The parent template.
    #[serde(flatten)]
    parent: ConfirmationEmail,
}

/// The plaintext version for email systems that do not support HTML.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConfirmationEmailText {
    /// The parent template.
    #[serde(flatten)]
    parent: ConfirmationEmail,
}

impl Template for ConfirmationEmailHtml {
    const TEMPLATE_NAME: &'static str = "emails/invites/html";
}

impl Template for ConfirmationEmailText {
    const TEMPLATE_NAME: &'static str = "emails/invites/text";
}

/// Structure to hold the template data passed to each template
/// that renders new user emails.
///
/// Has additional functions to modify an email object automatically.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmationEmail {
    /// The link the user should go to set their name and password.
    confirm_link: String,
    /// The invite id.
    invite_id: Uuid,
    /// When the invite expires (UTC)
    utc_expires: String,
    /// When the invite expires (EST)
    local_expires: String,
}

impl ConfirmationEmail {
    /// Construct a new user invite email. The domain may be pulled from the
    /// request uri. It should not have a `/` at the end of it.
    pub fn new(domain: impl Into<String>, invite: &Confirmation) -> Self {
        let local_offset = FixedOffset::east(time::UtcOffset::current_local_offset().as_seconds());
        let domain = domain.into();
        let invite_id = invite.invite_id.to_hyphenated().to_string().to_lowercase();
        Self {
            confirm_link: format!("{}/confirm/{}", domain.as_str(), invite_id),
            invite_id: invite.invite_id,
            utc_expires: invite.expiration.to_rfc2822(),
            local_expires: invite.expiration.with_timezone(&local_offset).to_rfc2822(),
        }
    }

    /// Make a plaintext clone.
    fn make_plaintext(&self) -> ConfirmationEmailText {
        ConfirmationEmailText {
            parent: self.clone(),
        }
    }

    /// Make HTML clone.
    fn make_html(&self) -> ConfirmationEmailHtml {
        ConfirmationEmailHtml {
            parent: self.clone(),
        }
    }

    /// Render the plaintext and HTML versions of this email and store them
    /// in the body of the email builder object.
    ///
    /// Panics if there are issues rendering either variant of the email.
    pub fn write_email(
        &self,
        ctx: &RequestContext,
        email: EmailBuilder,
    ) -> Result<EmailBuilder, String> {
        let registry = ctx.handlebars();
        let plaintext = self.make_plaintext().render(registry).map_err(|e| {
            error!("Could not render plaintext confirmation email: {}", e);
            "Plaintext email rendering error".to_string()
        })?;

        let html = self.make_html().render(registry).map_err(|e| {
            error!("Could not render HTML confirmation email: {}", e);
            "HTML email rendering error".to_string()
        })?;

        Ok(email.alternative(html, plaintext))
    }
}
