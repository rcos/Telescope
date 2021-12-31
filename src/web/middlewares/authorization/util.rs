//! Utility functions to help in the creation of authorization middlewares.

use crate::error::TelescopeError;
use crate::web::services::auth::identity::AuthenticationCookie;
use actix_identity::RequestIdentity;
use actix_web::dev::ServiceRequest;
use uuid::Uuid;

/// Extract the RCOS user ID authenticated with a request or error.
pub async fn extract_user_id(req: &ServiceRequest) -> Result<Uuid, TelescopeError> {
    req
        // Get the identity of the service request -- this should be a json encoded authentication
        // cookie if it exists.
        .get_identity()
        // Deserialize the authentication cookie object if it exists.
        .and_then(|ident| serde_json::from_str::<AuthenticationCookie>(ident.as_str()).ok())
        // If not authenticated, return an error indicating so.
        .ok_or(TelescopeError::NotAuthenticated)?
        // Refresh the cookie if necessary.
        .refresh()
        .await?
        // Get the RCOS user ID associated with the authenticated user.
        .get_user_id()
        .await?
        // Respond with an error if the user is not found.
        .ok_or(TelescopeError::NotAuthenticated)
}
