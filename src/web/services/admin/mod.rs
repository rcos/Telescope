//! Services for the admin panel.

use actix_web::web::ServiceConfig;
use crate::web::services::auth::identity::AuthenticationCookie;
use crate::templates::Template;
use crate::error::TelescopeError;
use crate::api::rcos::users::role_lookup::RoleLookup;
use crate::api::rcos::users::UserRole;
use actix_web::HttpRequest;

/// Register admin panel services.
pub fn register(config: &mut ServiceConfig) {
    config.service(index);
}

/// Admin page index.
#[get("/admin")]
async fn index(req: HttpRequest, auth: AuthenticationCookie) -> Result<Template, TelescopeError> {
    // Get the viewers username.
    let viewer: String = auth.get_rcos_username_or_error().await?;
    // Get the viewers role.
    let role: UserRole = RoleLookup::get(viewer)
        .await?
        // We can unwrap this because if the viewer has a username, their account exists.
        .expect("Viewer's account exists");

    // If the user is not an admin they are forbidden from viewing the admin page.
    if !role.is_admin() {
        return Err(TelescopeError::Forbidden);
    }

    // Otherwise return the admin page (currently just a static template).
    return Template::new("admin/index")
        // Rendered in a page of course.
        .render_into_page(&req, "RCOS Admin")
        .await;
}
