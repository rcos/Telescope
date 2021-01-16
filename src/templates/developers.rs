use crate::models::users::User;
use crate::templates::Template;
use crate::web::DbConnection;

/// A user thumbnail on the developer's page.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserThumbnail {
    /// The user object. Contains info like name, id.
    user: User,
    /// The resolved URL to get the profile picture from.
    pic_location: String,
}

impl UserThumbnail {
    /// Create the thumbnail for a user.
    pub async fn for_user(u: User, conn: DbConnection) -> Self {
        let pic_location: String = u.picture_url(conn).await;

        Self {
            user: u,
            pic_location,
        }
    }
}

/// The developer's page.
#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopersPage {
    users: Vec<UserThumbnail>,
}

impl DevelopersPage {
    /// The path to the handlebars file from the templates directory.
    pub const TEMPLATE_PATH: &'static str = "developers";

    /// The title of the web page.
    pub const PAGE_TITLE: &'static str = "RCOS Developers";

    /// Create the developers page template.
    pub fn new(thumbs: Vec<UserThumbnail>) -> Self {
        Self { users: thumbs }
    }

    /// Convert to a render-able template.
    pub fn template(&self) -> Template {
        Template::new(Self::TEMPLATE_PATH).with_fields(self)
    }
}
