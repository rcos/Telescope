use crate::web::Template;

/// Information displayed on a user profile.
#[derive(Clone, Serialize, Deserialize)]
pub struct Profile {
    /// User's name
    pub name: String,
    /// Can this profile be edited by the logged in user / viewer.
    pub editable: bool,
    /// User's profile picture. A default is generated using Gravatar.
    pub picture: String,
    /// User's bio rendered from markdown to html.
    pub bio: String,
}

impl Template for Profile {
    const TEMPLATE_NAME: &'static str = "profile";
}
