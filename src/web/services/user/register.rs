use crate::api::github::users::authenticated_user::authenticated_user::AuthenticatedUserViewer;
use crate::api::rcos::send_query;
use crate::api::rcos::users::create::{
    create_one_user::Variables as CreateOneUserVariables, CreateOneUser,
};
use crate::api::rcos::users::{UserAccountType, UserRole};
use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use crate::templates::{auth, page, Template};
use crate::web::profile_for;
use crate::web::services::auth::identity::{AuthenticationCookie, RootIdentity};
use crate::web::services::auth::rpi_cas::RpiCasIdentity;
use actix_web::http::header::LOCATION;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serenity::model::user::CurrentUser;
use std::collections::HashMap;
use actix_web::web::Form;

/// The path from the templates directory to the registration template.
const TEMPLATE_PATH: &'static str = "forms/register";

/// Form submitted by users when creating an account.
#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationFormInput {
    /// The new user's first name
    first_name: String,

    /// The new user's last name
    last_name: String,
}

impl RegistrationFormInput {
    /// Check that neither the first name or last name is empty.
    fn is_valid(&self) -> bool {
        !self.first_name.is_empty() && !self.last_name.is_empty()
    }
}

/// Create an empty registration form.
async fn empty_registration_form(id: &RootIdentity) -> Result<FormTemplate, TelescopeError> {
    // Create the base form
    let mut form = FormTemplate::new(TEMPLATE_PATH, "Create Account");

    // Build the form out with info depending on the root identity.
    match id {
        RootIdentity::Discord(d) => {
            form.template = d
                .get_authenticated_user()
                .await
                .map(|discord_user| json!({
                    "icon": UserAccountType::Discord,
                    "info": {
                        "username": discord_user.tag(),
                        "avatar_url": discord_user.face(),
                    }
                }))?;
        },

        RootIdentity::GitHub(g) => {
            form.template = g
                // Get the authenticated user
                .get_authenticated_user()
                .await
                // Convert the info to a JSON object as necessary
                .map(|gh_user| json!({
                    "icon": UserAccountType::GitHub,
                    "info": {
                        "username": gh_user.login,
                        "avatar_url": gh_user.avatar_url,
                        "profile_url": gh_user.url
                    }
                }))?;
        },

        RootIdentity::RpiCas(r) => {
            form.template = json!({
                "info": {
                    "username": format!("{}@rpi.edu", r.rcs_id),
                }
            });
        }
    }

    return Ok(form);
}

/// Function to construct a form with existing invalid user input.
async fn form_with_input(id: &RootIdentity, input: &RegistrationFormInput) -> Result<FormTemplate, TelescopeError> {
    // Create the empty form.
    let mut form = empty_registration_form(id).await?;

    // Get a mutable reference to the json value of the form's template
    let template = form.template
        .as_object_mut()
        .expect("The previous function should return a JSON object.");

    // Add the first and last name to the template.
    template.insert("first_name".into(), json!({
        "value": input.first_name,
        "error": input.first_name
            .is_empty()
            .then(|| "Your first name cannot be empty.")
            .unwrap_or("")
    }));

    template.insert("last_name".into(), json!({
        "value": input.last_name,
        "error": input.last_name
            .is_empty()
            .then(|| "Your last name cannot be empty.")
            .unwrap_or("")
    }));

    return Ok(form);
}

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
    req: HttpRequest,
    identity_cookie: AuthenticationCookie,
) -> Result<HttpResponse, actix_web::Error> {
    // If this authenticated identity is already linked to an account
    if let Some(rcs_id) = identity_cookie.get_rcos_username().await? {
        return Ok(HttpResponse::Found()
            .header(LOCATION, profile_for(rcs_id.as_str()))
            .finish());
    } else {
        // Otherwise create a form for the authenticated the user's cookie.
        // And convert it to an HttpResponse
        return empty_registration_form(&identity_cookie.root)
            .await
            .respond_to(&req)
            .await;
    }
}

#[post("/register/finish")]
/// Endpoint to which users submit their forms. Argument extractor will error if user is not
/// authenticated.
pub async fn submit_registration(
    identity_cookie: AuthenticationCookie,
    form_input: Form<RegistrationFormInput>,
) -> Result<HttpResponse, TelescopeError> {
    // Check if the form is valid.
    if !form_input.is_valid() {
        // If not return the invalid form.
        let form = form_with_input(&identity_cookie.root, &form_input).await?;
        return Err(TelescopeError::invalid_form(&form));
    }

    // Deconstruct the input.
    let RegistrationFormInput {first_name, last_name} = form_input.0;

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

        // On RPI CAS based identity
        RootIdentity::RpiCas(RpiCasIdentity { rcs_id }) => CreateOneUser::make_variables(
            rcs_id.clone(),
            first_name,
            last_name,
            // We assume anyone signing up via RPI CAS is a student.
            UserRole::Student,
            UserAccountType::Rpi,
            rcs_id.clone(),
        ),
    };

    // Extract the platform for use in error reporting if necessary.
    let platform: UserAccountType = query_vars.platform;

    // Create the account!
    // We have no subject field since the account isn't created until this request resolves
    let profile: String = send_query::<CreateOneUser>(query_vars)
        .await
        // If we cannot create an account, someone has probably already
        // linked the identity provider to another account. Tell the user to
        // cancel and try to login.
        .map_err(|_| TelescopeError::BadRequest {
            header: "Could Not Create Account".into(),
            message: format!(
                "We could not create an account. This likely (although not always) \
            means that your {0} account is already linked to an existing user's account. Please \
            try to login to that account. If you continue having issues or are sure that your {0} \
            account is not already linked to an existing user, please contact a coordinator and \
            file an issue on the Telescope GitHub.",
                platform
            ),
            show_status_code: false,
        })?
        // Extract the username
        .username()
        // Get the profile address
        .map(|username: String| profile_for(username.as_str()))
        // If there is no username, throw an error
        .ok_or(TelescopeError::ise(
            "Create User mutation did not return username",
        ))?;

    // Redirect the user to the account we created for them
    Ok(HttpResponse::Found().header(LOCATION, profile).finish())
}
