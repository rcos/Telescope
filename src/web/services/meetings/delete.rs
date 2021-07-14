//! Services for deleting meetings.

use actix_web::web::{ServiceConfig, Path};
use crate::web::services::auth::identity::AuthenticationCookie;
use actix_web::HttpResponse;
use crate::error::TelescopeError;

/// Register meeting deletion services.
pub fn register(config: &mut ServiceConfig) {
    config
        .service(delete_meeting);
}


#[get("/meeting/{meeting_id}/delete")]
async fn delete_meeting(auth: AuthenticationCookie, Path( meeting_id ): Path<i64>) -> Result<HttpResponse, TelescopeError> {
    return Err(TelescopeError::NotImplemented);
}
