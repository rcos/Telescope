use crate::error::TelescopeError;
use crate::templates::{auth, page, Template};
use actix_web::{HttpRequest, HttpResponse};
use crate::web::services::auth::identity::IdentityCookie;
use crate::templates::forms::{Form, register, FormInput};
use std::collections::HashMap;
use crate::web::api::rcos::users::create::CreateOneUser;
use crate::web::api::rcos::users::{UserAccountType, UserRole};
use crate::web::api::rcos::{send_query, make_api_client};
use actix_web::client::Client;
use actix_web::http::header::LOCATION;

#[get("/register")]
/// Service for the registration page. This page allows users to start the
/// account creation process by signing into an identity provider.
pub async fn register_page(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Make the create account page template.
    let content: Template = auth::register();
    // Put it in a page template and return it.
    return page::of(&req, "Create RCOS Account", &content).await;
}

#[get("/register/finish")]
/// Users finish the registration process by supplying their first and last name. Telescope creates
/// the necessary records in the RCOS database via the central API. Argument extractors will error
/// if the identity is not authenticated.
pub async fn finish_registration(identity_cookie: IdentityCookie) -> Result<Form, TelescopeError> {
    // Create a form for the authenticated the user's cookie.
    register::for_identity(&identity_cookie).await
}

#[post("/register/finish")]
/// Endpoint to which users submit their forms. Argument extractor will error if user is not
/// authenticated.
pub async fn submit_registration(identity_cookie: IdentityCookie, form_input: FormInput) -> Result<HttpResponse, TelescopeError> {
    // Create and validate a registration form. This will send the form back to the users repeatedly until they submit
    // valid input.
    let valid_form_input: HashMap<String, String> = register::for_identity(&identity_cookie)
        .await?
        .validate_input(form_input)
        .await?;

    // Extract the first and last name from the validated form input
    let first_name: String = valid_form_input.get(register::FNAME_FIELD)
        .expect("Form should have validated first name.")
        .clone();
    let last_name: String= valid_form_input.get(register::LNAME_FIELD)
        .expect("Form should have validated last name.")
        .clone();

    // Get the platform id username from the user's identity cookie.
    let platform: UserAccountType = identity_cookie.user_account_type();
    let username: String = identity_cookie.get_username_string().await?;
    let platform_id: String = identity_cookie.get_account_identity().await?;

    // Create central API mutation variables.
    let vars = CreateOneUser::make_variables(
        username,
        first_name,
        last_name,
        // All users are marked as external until linking an RPI CAS account.
        UserRole::External,
        platform,
        platform_id
    );

    // Make central RCOS API client.
    // We have no subject header yet, since the user account hasn't been created at this point.
    let api_client: Client = make_api_client(None);

    // Create the account!
    let created_username: String = send_query::<CreateOneUser>(&api_client, vars)
        .await?
        .username()
        .ok_or(TelescopeError::ise("Create User mutation did not return username"))?;

    // Redirect the user to the account we created for them
    Ok(HttpResponse::Found().header(LOCATION, format!("/users/{}", created_username)).finish())
}
