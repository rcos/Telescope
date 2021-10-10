use actix_web::web::{Form, Query, ServiceConfig};

use crate::{error::TelescopeError, templates::forms::FormTemplate, web::services::auth::identity::AuthenticationCookie};

async fn get_context_and_make_form(
    auth: &AuthenticationCookie,
) -> Result<FormTemplate, TelescopeError> {
    let viewer = auth.get_rcos_username_or_error().await?;

    Err(TelescopeError::NotImplemented)
}

#[get("/manage_enrollments")]
pub async fn manage_page(auth: AuthenticationCookie) -> Result<FormTemplate, TelescopeError> {
    get_context_and_make_form(&auth).await
}
