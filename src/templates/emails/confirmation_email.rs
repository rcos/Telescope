use uuid::Uuid;
use crate::web::Template;

/// The HTML version of the email sent to new users asking them
/// to go to the telescope website and set their name and create a password.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConfirmationEmailHtml {
    /// The parent template.
    #[serde(flatten)]
    parent: ConfirmationEmail
}

/// The plaintext version for email systems that do not support HTML.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConfirmationEmailText {
    /// The parent template.
    #[serde(flatten)]
    parent: ConfirmationEmail
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
    /// The domain telescope is running at.
    /// Used to retrieve images for the html version.
    domain: String,
    /// The link the user should go to set their name and password.
    confirm_link: String,
    /// The invite id.
    invite_id: Uuid
}

impl ConfirmationEmail {
    /// Construct a new user invite email. The domain may be pulled from the
    /// request uri. It should not have a `/` at the end of it.
    pub fn new(domain: impl Into<String>, invite_id: Uuid) -> Self {
        let domain = domain.into();
        Self {
            confirm_link: format!("{}/confirm/{}",
                                  domain.as_str(),
                                  invite_id.to_hyphenated().to_string().to_lowercase()),
            domain,
            invite_id
        }
    }
}
