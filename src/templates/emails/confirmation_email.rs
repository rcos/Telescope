use crate::{models::confirmations::Confirmation, templates::Template, web::RequestContext};
use chrono::Local;
use handlebars::Handlebars;
use lettre_email::EmailBuilder;
use uuid::Uuid;

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
    /// Plaintext template path from template root.
    const PLAINTEXT_TEMPLATE: &'static str = "emails/invites/text";

    /// HTML template path from template root.
    const HTML_TEMPLATE: &'static str = "emails/invites/html";

    /// Construct a new user invite email. The domain may be pulled from the
    /// request uri. It should not have a `/` at the end of it.
    pub fn new(domain: impl Into<String>, invite: &Confirmation) -> Self {
        let domain = domain.into();
        let invite_id = invite.invite_id.to_hyphenated().to_string().to_lowercase();
        Self {
            confirm_link: format!("{}/confirm/{}", domain.as_str(), invite_id),
            invite_id: invite.invite_id,
            utc_expires: invite.expiration.to_rfc2822(),
            local_expires: invite.expiration.with_timezone(&Local).to_rfc2822(),
        }
    }

    /// Make a plaintext clone.
    fn make_plaintext(&self) -> Template {
        Template::new(Self::PLAINTEXT_TEMPLATE).with_fields(self)
    }

    /// Make HTML clone.
    fn make_html(&self) -> Template {
        Template::new(Self::HTML_TEMPLATE).with_fields(self)
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
        let registry: &Handlebars = ctx.handlebars();
        let plaintext: String = self.make_plaintext().render(registry);
        let html: String = self.make_html().render(registry);
        Ok(email.alternative(html, plaintext))
    }
}
