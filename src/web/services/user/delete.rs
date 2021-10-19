use actix_web::{HttpRequest, HttpResponse, web::Form, http::header::LOCATION};
use crate::api::rcos::users::profile::Profile;
use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use crate::web::services::auth::identity::{AuthenticationCookie, Identity};

// Confirmation form to delete the profile
#[get("/profile_delete")]
pub async fn confirm_delete(auth: AuthenticationCookie) -> Result<FormTemplate, TelescopeError> {
    let username = auth.get_rcos_username_or_error().await?;
    let profiledata = dbg!(Profile::for_user(username.clone(), Some(username)).await?);

    let mut form = FormTemplate::new("user/delete", "Delete confirmation");
    form.template = json!(profiledata);
    dbg!(form.template.to_string());

    Ok(form)
}

#[post("/profile_delete")]
pub async fn profile_delete(identity: Identity) -> Result<HttpResponse, TelescopeError> {
    identity.forget();
    return Ok(HttpResponse::Found()
              .header(LOCATION, "/")
              .finish());
}
