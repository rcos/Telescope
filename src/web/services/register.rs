use crate::error::TelescopeError;
use crate::templates::forms::{register, Form, FormInput};
use crate::templates::{auth, page, Template};
use crate::web::api::github::users::authenticated_user::authenticated_user::AuthenticatedUserViewer;
use crate::web::api::rcos::send_query;
use crate::web::api::rcos::users::create::{
    create_one_user::Variables as CreateOneUserVariables, CreateOneUser,
};
use crate::web::api::rcos::users::{UserAccountType, UserRole};
use crate::web::services::auth::identity::{AuthenticatedIdentities, RootIdentity};
use actix_web::http::header::LOCATION;
use actix_web::{HttpRequest, HttpResponse};
use serenity::model::user::CurrentUser;
use std::collections::HashMap;
use crate::web::profile_for;

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
pub async fn finish_registration(
    identity_cookie: AuthenticatedIdentities,
) -> Result<Form, TelescopeError> {
    // Create a form for the authenticated the user's cookie.
    register::for_identity(&identity_cookie.root).await
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
    let valid_form_input: HashMap<String, String> = register::for_identity(&identity_cookie.root)
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

    // Make query variables to create user
    let query_vars: CreateOneUserVariables = match &identity_cookie.root {
        // On GitHub authenticated identity
        RootIdentity::GitHub(gh) => {
            // Get the authenticated user.
            let authenticated_user: AuthenticatedUserViewer = gh.get_authenticated_user().await?;
            // Destructure important fields
            let AuthenticatedUserViewer { login, id, .. } = authenticated_user;
            // Build query variables.
            CreateOneUser::make_variables(
                login,
                first_name,
                last_name,
                UserRole::External,
                UserAccountType::GitHub,
                id,
            )
        }

        // On Discord authenticated identity.
        RootIdentity::Discord(d) => {
            // Get authenticated user
            let authenticated_user: CurrentUser = d.get_authenticated_user().await?;
            // Build query variables.
            CreateOneUser::make_variables(
                authenticated_user.tag(),
                first_name,
                last_name,
                UserRole::External,
                UserAccountType::Discord,
                authenticated_user.id.to_string(),
            )
        }
    };

    // Create the account!
    // We have no subject field since the account isn't created until this request resolves
    let profile: String = send_query::<CreateOneUser>(None, query_vars)
        .await?
        // Extract the username
        .username()
        // Get the profile address
        .map(|username: String| profile_for(username.as_str()))
        // If there is no username, throw an error
        .ok_or(TelescopeError::ise(
            "Create User mutation did not return username",
        ))?;

    // Redirect the user to the account we created for them
    Ok(HttpResponse::Found()
        .header(LOCATION, profile)
        .finish())
}
