//! Profile services.

use crate::web::services::auth::identity::AuthenticationCookie;
use actix_web::web::Path;
use crate::templates::Template;
use crate::error::TelescopeError;
use crate::templates::user::profile as profile_template;
use actix_web::HttpRequest;
use crate::web::api::rcos::users::profile::{
    Profile,
    profile::{
        ResponseData,
        ProfileUsersByPk
    }
};
use crate::templates::user::profile::TargetUser;

/// Wrapper struct for deserializing username.
#[derive(Serialize, Deserialize)]
struct SerializedUsername { u: String }

/// User profile service. The username in the path is url-encoded.
#[get("/user/{username}")]
pub async fn profile(
    req: HttpRequest,
    authentication: Option<AuthenticationCookie>,
    Path(username): Path<String>
) -> Result<Template, TelescopeError> {
    // Decode the url-encoded username.
    let decoded_username: String = serde_urlencoded::from_str::<SerializedUsername>(format!("u={}", username).as_str())
        // Convert the error and get the inner value.
        .map_err(|_| TelescopeError::bad_request(
            "Malformed Username",
            "Username not properly URL encoded."
        ))?.u;

    // Get the user's profile information from the RCOS API.
    let target_user: TargetUser = Profile::for_user(decoded_username)
        .await?
        .users_by_pk
        .ok_or(TelescopeError::resource_not_found(
            "User Not Found",
            "Could not find a user by this username."
        ))?
        .into();


    // Make a profile template
    return profile_template::make(&target_user)
        // Render it inside a page (with the user's name as the title)
        .render_into_page(&req, target_user.name.as_str())
        // Wait for the page to render and return
        .await;
}
