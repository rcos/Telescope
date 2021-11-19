//! Enrollments services and functionality.

use actix_web::web::ServiceConfig;
use uuid::Uuid;
use actix_web::{HttpResponse, Resource, web as aweb};
use crate::api::rcos::users::accounts::lookup::AccountLookup;
use crate::api::rcos::users::UserAccountType;
use crate::error::TelescopeError;
use crate::templates::Template;
use crate::web::middlewares::authorization::Authorization;
use crate::web::services::auth::identity::AuthenticationCookie;

/// Register enrollment related endpoints.
pub fn register(conf: &mut ServiceConfig) {
    // Create middleware to require that all users enrolling in RCOS have an RCS
    // ID linked.
    let enrollment_auth = Authorization::new(|user_id: Uuid| {
        Box::pin(async move {
            // Try to get user's RCS ID.
            let rcs_id: Option<String> = AccountLookup::send(user_id, UserAccountType::Rpi)
                .await?;

            // Return error if there is no RCS ID linked to an account.
            rcs_id.ok_or(TelescopeError::BadRequest {
                header: "Enrollment requires RCS ID to be linked.".into(),
                message: "Please link your RCS ID before you enroll.".into(),
                show_status_code: false
            })?;

            Ok(())
        })
    });

    // Make the enrollment resource.
    let enrollment_resource = Resource::new("/enroll")
        .wrap(enrollment_auth)
        .route(aweb::get().to(enrollment_form))
        .route(aweb::post().to(save_enrollment));

    // Register enrollment resource.
    conf.service(enrollment_resource);
}


/// Endpoint to show users the enrollment form.
async fn enrollment_form(auth: AuthenticationCookie) -> Result<Template, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}

/// Endpoint to accept enrollment form submissions.
async fn save_enrollment(auth: AuthenticationCookie) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}
