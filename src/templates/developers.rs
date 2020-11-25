use crate::models::users::User;

/// A user thumbnail on the developer's page.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserThumbnail {}

/// The developer's page.
#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopersPage {
    users: Vec<UserThumbnail>,
}

