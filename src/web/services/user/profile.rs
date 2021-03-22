//! Profile services.

use crate::web::services::auth::identity::AuthenticationCookie;
use actix_web::web::Path;
use crate::templates::Template;
use crate::error::TelescopeError;
use crate::templates::user::profile as profile_template;
use actix_web::HttpRequest;

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
        .map_err(|_| TelescopeError::bad_request(
            "Malformed Username",
            "Username not properly URL encoded."
        ))?.u;

    // Make a profile template
    return profile_template::make(decoded_username.clone())
        // Render it inside a page
        .render_into_page(&req, decoded_username.as_str())
        // Wait for the page to render and return
        .await;
}
