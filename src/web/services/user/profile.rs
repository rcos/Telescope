//! Profile services.

use crate::error::TelescopeError;
use crate::templates::user::profile as profile_template;
use crate::templates::Template;
use crate::web::api::rcos::users::profile::{
    profile::{ProfileTarget, ProfileTargetCoordinating, ProfileTargetMentoring, ResponseData},
    Profile,
};
use crate::web::services::auth::identity::{AuthenticationCookie, Identity};
use actix_web::web::Query;
use actix_web::HttpRequest;

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
    Query(ProfileQuery { username }): Query<ProfileQuery>
) -> Result<Template, TelescopeError> {
    // Get the user's profile information from the RCOS API.
    let response: ResponseData = Profile::for_user(username.clone()).await?;
    // Throw an error if there is no user.
    if response.target.is_none() {
        return Err(TelescopeError::resource_not_found(
            "User Not Found",
            "Could not find a user by this username.",
        ));
    }

    // Get the target user's info.
    let target_user: ProfileTarget = response.target.unwrap();
    // Get the viewer's username.
    let viewer_username: Option<String> = identity.get_rcos_username().await?;

    // Make a profile template
    // Render it inside a page (with the user's name as the title)
    return profile_template::make(&target_user, viewer_username)
        .render_into_page(&req, format!("{} {}", target_user.first_name, target_user.last_name))
        .await;
}
