use actix_web::web::{Form, Query, ServiceConfig};

use crate::api::rcos::enrollments;
use crate::{error::TelescopeError, templates::forms::FormTemplate, web::services::auth::identity::AuthenticationCookie};
use crate::api::rcos::enrollments::get::{self, Enrollments};

async fn get_context_and_make_form(
    auth: &AuthenticationCookie,
) -> Result<FormTemplate, TelescopeError> {
    let viewer = auth.get_rcos_username_or_error().await?;
    let data = dbg!(enrollments::get::Enrollments::get(viewer, 0).await?);

    let mut form = FormTemplate::new("user/enrollments", "Manage Enrollments");
    form.template = json!({
        "enrollments": data.enrollments
    });

    return Ok(form);
}

#[get("/manage_enrollments")]
pub async fn manage_page(auth: AuthenticationCookie) -> Result<FormTemplate, TelescopeError> {
    get_context_and_make_form(&auth).await
}
