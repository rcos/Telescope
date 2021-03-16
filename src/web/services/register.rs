use crate::error::TelescopeError;
use crate::templates::forms::{register, Form, FormInput};
use crate::templates::{auth, page, Template};
use crate::web::api::rcos::send_query;
use crate::web::api::rcos::users::create::CreateOneUser;
use crate::web::api::rcos::users::{UserAccountType, UserRole};
use crate::web::services::auth::identity::AuthenticatedIdentities;
use actix_web::http::header::LOCATION;
use actix_web::{HttpRequest, HttpResponse};
use std::collections::HashMap;

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
pub async fn finish_registration(identity_cookie: AuthenticatedIdentities) -> Result<Form, TelescopeError> {
    // Create a form for the authenticated the user's cookie.
    register::for_identity(&identity_cookie).await
}

#[post("/register/finish")]
/// Endpoint to which users submit their forms. Argument extractor will error if user is not
/// authenticated.
pub async fn submit_registration(
    identity_cookie: AuthenticatedIdentities,
    form_input: FormInput,
) -> Result<HttpResponse, TelescopeError> {
    // Create and validate a registration form. This will send the form back to the users repeatedly until they submit
    // valid input.
    let valid_form_input: HashMap<String, String> = register::for_identity(&identity_cookie)
        .await?
        .validate_input(form_input)
        .await?;

    // Extract the first and last name from the validated form input
    let first_name: String = valid_form_input
        .get(register::FNAME_FIELD)
        .expect("Form should have validated first name.")
        .clone();
    let last_name: String = valid_form_input
        .get(register::LNAME_FIELD)
        .expect("Form should have validated last name.")
        .clone();

    // Get the platform id username from the user's identity cookie.
    let platform: UserAccountType;
    let username: String;
    let platform_id: String;

    // Check for discord identity first.
    if let Some(discord) = identity_cookie.discord {
        let user = discord.get_authenticated_user().await?;
        platform = UserAccountType::Discord;
        username = user.tag();
        platform_id = user.id.to_string();
    } else {
        // If no discord identity, use github.
        let github_user = identity_cookie.github
            .expect("At least one identity provider should be defined")
            .get_authenticated_user()
            .await?;

        platform = UserAccountType::GitHub;
        platform_id = github_user.id;
        username = github_user.login;
    }

    // Create central API mutation variables.
    let vars = CreateOneUser::make_variables(
        username,
        first_name,
        last_name,
        // All users are marked as external until linking an RPI CAS account.
        UserRole::External,
        platform,
        platform_id,
    );

    // Create the account!
    // We have no subject field since the account isn't created until this request resolves
    let created_username: String = send_query::<CreateOneUser>(None, vars)
        .await?
        .username()
        .ok_or(TelescopeError::ise(
            "Create User mutation did not return username",
        ))?;

    // Redirect the user to the account we created for them
    Ok(HttpResponse::Found()
        .header(LOCATION, format!("/users/{}", created_username))
        .finish())
}
