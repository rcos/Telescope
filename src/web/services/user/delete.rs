use crate::api::rcos::users::{delete::DeleteUser, profile::Profile};
use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use crate::web::services::auth::identity::{AuthenticationCookie, Identity};
use actix_web::{http::header::LOCATION, web::Form, HttpRequest, HttpResponse};

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
    DeleteUser::execute(identity.get_rcos_username().await.map(
        |x| -> Result<String, TelescopeError> {
            x.ok_or(TelescopeError::InternalServerError(
                "Missing username".to_string(),
            ))
        },
    )??).await?;
    identity.forget();
    return Ok(HttpResponse::Found().header(LOCATION, "/?notice=Your%20account%20was%20deleted%20successfully.").finish());
}
