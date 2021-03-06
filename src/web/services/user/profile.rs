//! Profile services.

use crate::api::rcos::users::profile::{
    profile::{ProfileTarget, ResponseData},
    Profile,
};
use crate::error::TelescopeError;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use actix_web::web::Query;
use actix_web::HttpRequest;

/// The path from the template directory to the profile template.
const TEMPLATE_NAME: &'static str = "user/profile";

/// Wrapper struct for deserializing username.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfileQuery {
    /// The username of the owner of the profile.
    pub username: String,
}

/// User profile service. The username in the path is url-encoded.
#[get("/user")]
pub async fn profile(
    req: HttpRequest,
    identity: Identity,
    // TODO: Switch to using Path here when we switch to user ids.
    Query(ProfileQuery { username }): Query<ProfileQuery>,
) -> Result<Template, TelescopeError> {
    // Get the viewer's username.
    let viewer_username: Option<String> = identity.get_rcos_username().await?;

    // Get the user's profile information (and viewer info) from the RCOS API.
    let response: ResponseData = Profile::for_user(username, viewer_username).await?;

    // Throw an error if there is no user.
    if response.target.is_none() {
        return Err(TelescopeError::resource_not_found(
            "User Not Found",
            "Could not find a user by this username.",
        ));
    }

    // Get the target user's info.
    let target_user: &ProfileTarget = response.target.as_ref().unwrap();
    // And use it to make the page title
    let page_title = format!("{} {}", target_user.first_name, target_user.last_name);

    // Get the target user's discord info.

    // Make a profile template
    return Template::new(TEMPLATE_NAME)
        .field("data", response)
        // Render it inside a page (with the user's name as the title)
        .render_into_page(&req, page_title)
        .await;
}
