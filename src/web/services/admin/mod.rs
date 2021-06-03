//! Services for the admin panel.

mod semesters;
mod middleware;

use actix_web::web::ServiceConfig;
use crate::web::services::auth::identity::AuthenticationCookie;
use crate::templates::Template;
use crate::error::TelescopeError;
use crate::api::rcos::users::role_lookup::RoleLookup;
use crate::api::rcos::users::UserRole;
use actix_web::HttpRequest;
use actix_web::web as aweb;
use actix_web::dev::{ServiceRequest, Service};
use actix_identity::RequestIdentity;
use crate::web::services::admin::middleware::AdminAuthorization;
use actix_web::guard;

/// Register admin panel services.
pub fn register(config: &mut ServiceConfig) {
    // Admin panel index page.
    config.service(aweb::resource("/admin")
        .guard(guard::Get())
        .wrap(AdminAuthorization)
        .to(index));

    // Route every sub-service through the admin scope.
    config.service(
        // Create the admin scope.
        aweb::scope("/admin/")
            // Verify that the viewer has the admin role.
            .wrap(AdminAuthorization)

            // Semester services
            .configure(semesters::register)
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
