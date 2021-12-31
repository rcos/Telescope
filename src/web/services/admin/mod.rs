//! Services for the admin panel.

mod semesters;

use crate::api::rcos::users::role_lookup::RoleLookup;
use crate::api::rcos::users::UserRole;
use crate::error::TelescopeError;
use crate::middlewares::authorization::util::extract_user_id;
use crate::templates::page::Page;
use crate::templates::Template;
use crate::web::middlewares::authorization::{Authorization, AuthorizationResult};
use actix_web::dev::ServiceRequest;
use actix_web::guard;
use actix_web::web as aweb;
use actix_web::web::ServiceConfig;
use actix_web::HttpRequest;
use futures::future::LocalBoxFuture;
use uuid::Uuid;

/// Check that a user is an admin, in the form of an authorization middleware.
fn admin_auth_middleware(req: &ServiceRequest) -> LocalBoxFuture<AuthorizationResult> {
    Box::pin(async move {
        // Get the user ID.
        let user_id: Uuid = extract_user_id(req).await?;
        // Lookup the users role. This should never return None, since the user has to exist.
        let user_role: UserRole = RoleLookup::get(user_id)
            .await?
            .expect("Viewer's account does not exist.");

        // Forbid non-admin users.
        if !user_role.is_admin() {
            return Err(TelescopeError::Forbidden);
        }

        // Default to success
        Ok(())
    })
}

/// Register admin panel services.
pub fn register(config: &mut ServiceConfig) {
    // Create admin authorization middleware.
    let admin_authorization_middleware: Authorization = Authorization::new(admin_auth_middleware);

    // Admin panel index page.
    config.service(
        aweb::resource("/admin")
            .guard(guard::Get())
            .wrap(admin_authorization_middleware.clone())
            .to(index),
    );

    // Route every sub-service through the admin scope.
    config.service(
        // Create the admin scope.
        aweb::scope("/admin/")
            // Verify that the viewer has the admin role.
            .wrap(admin_authorization_middleware)
            // Semester services
            .configure(semesters::register),
    );
}

/// Admin page index.
async fn index(req: HttpRequest) -> Result<Page, TelescopeError> {
    // Access is pre-checked by the scope this is in.
    // Return the admin page (currently just a static template).
    return Template::new("admin/index")
        // Rendered in a page of course.
        .in_page(&req, "RCOS Admin")
        .await;
}
