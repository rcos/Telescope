use crate::{
    models::{markdown::render as md_render, users::User},
    web::{RequestContext, Template},
};
use chrono::FixedOffset;
use time::UtcOffset;

/// User profile template.
#[derive(Clone, Serialize, Deserialize)]
pub struct Profile {
    /// The user object that this template represents.
    user: User,
    /// Can this profile be edited by the logged in user / viewer.
    editable: bool,
    /// User's profile picture. A default is generated using Gravatar.
    picture: String,
    /// User's bio rendered from markdown to html.
    bio: String,
    /// Public emails to display on the profile.
    public_emails: Vec<String>,
    /// String representation of when the account was created.
    created_at: String,
}

impl Profile {
    /// Determine if user `a` should be able to edit the profile of user `b`.
    /// Currently this is only true if `a` is a sysadmin or is the same user as
    /// b.
    fn can_edit(a: &User, b: &User) -> bool {
        a.sysadmin || a.id == b.id
    }

    /// Create a template for a given user.
    /// Panics if database query fails.
    /// The profile is determined to be editable using the function above this.
    pub async fn for_user(user: User, ctx: &RequestContext) -> Self {
        // get emails.
        let emails = user.get_emails_from_db(ctx.get_db_conn().await).await;

        // select public emails to display
        let public_emails = emails
            .iter()
            .filter(|e| e.is_visible)
            .map(|e| e.email.clone())
            .collect::<Vec<String>>();

        // determine whether or not to make the profile editable.
        let editable = ctx
            .user_identity()
            .await
            .map(|viewer| Profile::can_edit(&viewer, &user))
            .unwrap_or(false);

        // determine the profile picture to show.
        let picture = user
            .avi_location
            .as_ref()
            .map(|s| s.to_string())
            // if no user specified one is available,
            // make a gravatar url from the first email
            // (there must be at least one).
            .unwrap_or_else(|| {
                emails
                    .first()
                    .map(|e| {
                        let email_str = e.email.as_str().trim().to_lowercase();
                        // make md5 hash of the email and build gravitar url
                        let gravatar_hash = md5::compute(email_str);
                        format!(
                            "https://www.gravatar.com/avatar/{:x}?d=identicon&s=600",
                            gravatar_hash
                        )
                    })
                    .expect("Could not get gravitar email.")
            });

        // render the user's bio
        let rendered_bio = md_render(user.bio.as_str());

        // make a string of the account creation time after converting to EST.
        let local_offset = FixedOffset::east(UtcOffset::current_local_offset().as_seconds());
        let localized_time = user
            .account_created
            .with_timezone(&local_offset)
            .format("%B %Y")
            .to_string();

        Self {
            user,
            editable,
            picture,
            bio: rendered_bio,
            public_emails,
            created_at: localized_time,
        }
    }
}

impl Template for Profile {
    const TEMPLATE_NAME: &'static str = "user/profile";
}
