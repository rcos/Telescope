//! Services for the admin panel.

mod semesters;

use crate::api::rcos::users::role_lookup::RoleLookup;
use crate::api::rcos::users::UserRole;
use crate::error::TelescopeError;
use crate::templates::Template;
use crate::web::middlewares::authorization::{Authorization, AuthorizationResult};
use actix_web::guard;
use actix_web::web as aweb;
use actix_web::web::ServiceConfig;
use actix_web::HttpRequest;
use futures::future::LocalBoxFuture;

/// Check that a user is an admin.
fn admin_authorization(username: String) -> LocalBoxFuture<'static, AuthorizationResult> {
    Box::pin(async move {
        // Then check that their role is admin.
        let role: UserRole = RoleLookup::get(username)
            .await?
            // The role should not be none, since the account needs to exist at this point.
            .expect("Viewer's account does not exist.");

        // Forbid access unless the user is an admin.
        if !role.is_admin() {
            Err(TelescopeError::Forbidden)
        } else {
            Ok(())
        }
    })
}

/// Register admin panel services.
pub fn register(config: &mut ServiceConfig) {
    // Create admin authorization middleware.
    let admin_authorization_middleware: Authorization = Authorization::new(admin_authorization);

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
async fn index(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Access is pre-checked by the scope this is in.
    // Return the admin page (currently just a static template).
    return Template::new("admin/index")
        // Rendered in a page of course.
        .render_into_page(&req, "RCOS Admin")
        .await;
}
