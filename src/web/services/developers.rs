//! Developers page services

use crate::templates::Template;
use crate::error::TelescopeError;

/// The developer catalogue. This page displays all of the users in the
/// RCOS database.
#[get("/developers")]
pub async fn developers_page() -> Result<Template, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}
