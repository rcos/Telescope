//! Profile services.

use crate::error::TelescopeError;
use crate::templates::user::profile as profile_template;
use crate::templates::Template;
use crate::web::api::rcos::users::profile::{
    profile::{ProfileTarget, ProfileTargetCoordinating, ProfileTargetMentoring, ResponseData},
    Profile,
};
use crate::web::services::auth::identity::AuthenticationCookie;
use actix_web::web::{Path, Query};
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
    authentication: Option<AuthenticationCookie>,
    // This is removed until we switch over to UUID usernames.
    //Path(username): Path<String>,
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

    // Get a reference to the user's info.
    let target_user: &ProfileTarget = response.target.as_ref().unwrap();
    // Get references to the parts of the user's info needed to build the profile template.
    let mentoring: &[ProfileTargetMentoring] = target_user.mentoring.as_slice();
    let coordinating: &[ProfileTargetCoordinating] = target_user.coordinating.as_slice();
    let name: String = format!("{} {}", target_user.first_name, target_user.last_name);
    // Determine viewer privileges
    let viewer_is_authenticated: bool = authentication.is_some();
    let viewer_owns_profile: bool;
    if let Some(viewer) = authentication {
        viewer_owns_profile = viewer.get_rcos_username_or_error().await? == username;
    } else {
        viewer_owns_profile = false;
    }

    //


    // Make a profile template
    // Render it inside a page (with the user's name as the title)
    return profile_template::make(
        name.as_str(),
        target_user.created_at,
        mentoring,
        coordinating,
    )
    .render_into_page(&req, name.as_str())
    .await;
}
