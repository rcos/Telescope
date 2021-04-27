use crate::error::TelescopeError;
use crate::web::api::rcos::users::accounts::for_user::UserAccounts;
use crate::web::api::rcos::users::accounts::unlink::UnlinkUserAccount;
use crate::web::api::rcos::users::UserAccountType;
use crate::web::profile_for;
use crate::web::services::auth::identity::{AuthenticationCookie, Identity};
use crate::web::services::auth::oauth2_providers::discord::DiscordOAuth;
use crate::web::services::auth::rpi_cas::RpiCas;
use actix_web::http::header::{HOST, LOCATION};
use actix_web::web::ServiceConfig;
use actix_web::{web as aweb, Responder};
use actix_web::{HttpRequest, HttpResponse};
use futures::future::LocalBoxFuture;
use oauth2::RedirectUrl;
use oauth2_providers::github::GitHubOauth;
use std::collections::HashMap;
use std::future::Future;

pub mod identity;
pub mod oauth2_providers;
pub mod rpi_cas;

/// The types of user accounts that provide authentication.
const AUTHENTICATOR_ACCOUNT_TYPES: [UserAccountType; 3] = [
    UserAccountType::Rpi,
    UserAccountType::GitHub,
    UserAccountType::Discord,
];

/// Register auth services.
pub fn register(config: &mut ServiceConfig) {
    // GitHub OAuth2 provider services.
    GitHubOauth::register_services(config);

    // Discord OAuth2 provider services.
    DiscordOAuth::register_services(config);

    // RPI CAS provider services.
    RpiCas::register_services(config);
}

/// Function to create the redirect URL for a given request and identity provider's
/// redirect path.
fn make_redirect_url(req: &HttpRequest, redir_path: String) -> RedirectUrl {
    // Get the host header to determine where to redirect the user to.
    // This should be the base for one of the identity provider's redirect
    // paths.
    let address: &str = req
        .headers()
        .get(HOST)
        .expect("Could not get host header from request.")
        .to_str()
        .expect("Host request header is not ascii characters");

    // Create and return redirect URL.
    return RedirectUrl::new(format!("https://{}{}", address, redir_path))
        .expect("Could not create redirect URL");
}

/// Trait for identity providers (GitHub OAuth2, Discord OAuth2, RPI CAS, etc).
pub trait IdentityProvider: 'static {
    /// The lowercase, one word name of the service. This is used in generating
    /// redirect paths and registering the service with actix. It must be unique.
    const SERVICE_NAME: &'static str;

    /// The type of user account represented by this authentication service.
    const USER_ACCOUNT_TY: UserAccountType;

    /// Get the login path of this service. This is the route in actix that will
    /// redirect to the authorization page using the handler function also defined
    /// in this trait.
    fn login_path() -> String {
        format!("/login/{}", Self::SERVICE_NAME)
    }

    /// Get the registration path of this service. This is the route in actix that
    /// will redirect to the authorization page using the handler also defined by
    /// this trait. This is similar to [`Self::login_path`] but is for account
    /// registration rather than sign in.
    fn register_path() -> String {
        format!("/register/{}", Self::SERVICE_NAME)
    }

    /// The path to link this identity service. This is similar to the other two,
    /// but is intended to be used to link an existing account.
    fn link_path() -> String {
        format!("/link/{}", Self::SERVICE_NAME)
    }

    /// The path to unlink this service from the user's account.
    fn unlink_path() -> String {
        format!("/unlink/{}", Self::SERVICE_NAME)
    }

    /// The path for the identity provider to redirect back to after authenticating
    /// a user for login. This path is also registered under actix with the
    /// authentication callback handler defined by this trait.
    fn login_redirect_path() -> String {
        format!("/auth/{}/login", Self::SERVICE_NAME)
    }

    /// The path for the identity provider to redirect back to after authenticating
    /// a user for account creation. This path is also registered under actix with
    /// the authentication callback handler defined by this trait.
    fn registration_redirect_path() -> String {
        format!("/auth/{}/register", Self::SERVICE_NAME)
    }

    /// The path to redirect back to after account linking success. This is
    /// similar to the login and registration authenticated redirect paths.
    fn link_redirect_path() -> String {
        format!("/auth/{}/link", Self::SERVICE_NAME)
    }

    /// The type used to respond to login requests.
    type LoginResponse: Responder;

    /// The type used to respond to registration requests.
    type RegistrationResponse: Responder;

    /// The type used to respond to account linking requests.
    type LinkResponse: Responder;

    /// The type of future returned by the login handler.
    type LoginFut: Future<Output = Self::LoginResponse> + 'static;

    /// The type of the future returned by the registration handler.
    type RegistrationFut: Future<Output = Self::RegistrationResponse> + 'static;

    /// The type of the future returned by the account linking handler.
    type LinkFut: Future<Output = Self::LinkResponse> + 'static;

    /// The type of future returned by the login authenticated response handler.
    type LoginAuthenticatedFut: Future<Output = Result<HttpResponse, TelescopeError>> + 'static;

    /// The type of future returned by the registration authenticated response handler.
    type RegistrationAuthenticatedFut: Future<Output = Result<HttpResponse, TelescopeError>>
        + 'static;

    /// The type of future returned by the registration authenticated response handler.
    type LinkAuthenticatedFut: Future<Output = Result<HttpResponse, TelescopeError>> + 'static;

    /// Register the necessary actix services to support this identity
    /// provider.
    fn register_services(config: &mut ServiceConfig) {
        config
            .route(
                Self::register_path().as_str(),
                aweb::get().to(Self::registration_handler),
            )
            .route(
                Self::login_path().as_str(),
                aweb::get().to(Self::login_handler),
            )
            .route(
                Self::link_path().as_str(),
                aweb::get().to(Self::link_handler),
            )
            .route(
                Self::unlink_path().as_str(),
                aweb::get().to(Self::unlink_handler),
            )
            .route(
                Self::login_redirect_path().as_str(),
                aweb::get().to(Self::login_authenticated_handler),
            )
            .route(
                Self::registration_redirect_path().as_str(),
                aweb::get().to(Self::registration_authenticated_handler),
            )
            .route(
                Self::link_redirect_path().as_str(),
                aweb::get().to(Self::linking_authenticated_handler),
            );
    }

    /// Actix-web handler for the route that redirects to authentication for
    /// login. Guarded by this trait to GET requests.
    fn login_handler(req: HttpRequest) -> Self::LoginFut;

    /// Actix-web handler for the route that redirects to authentication for
    /// account creation (user registration). Guarded by this
    /// trait to GET requests.
    fn registration_handler(req: HttpRequest) -> Self::RegistrationFut;

    /// Actix-web handler for the route that redirects to the authentication provider
    /// to link an account.
    ///
    /// Linking an account will authenticate with the identity provider and then insert
    /// the account into the RCOS database via the API. If there is already an account linked,
    /// then the account's platform ID will be checked against the ID of the linked account.
    /// If it matches then the auth cookie is modified, and we return a successful response.
    /// If it does not match, we forget the new account and tell the user to unlink their
    /// existing account on this platform first.
    fn link_handler(req: HttpRequest, ident: Identity) -> Self::LinkFut;

    /// Actix-web handler for the route that unlinks an identity service.
    fn unlink_handler(
        req: HttpRequest,
        id: Identity,
        mut cookie: AuthenticationCookie,
    ) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>> {
        return Box::pin(async move {
            // Lookup the username of the user trying to unlink an account.
            let username: String = cookie.get_rcos_username_or_error().await?;
            // Get all of the accounts linked to this user. Make sure at least one
            // can function for authentication.
            let all_accounts: HashMap<UserAccountType, String> =
                UserAccounts::send(username.clone())
                    .await?
                    // Iterate
                    .into_iter()
                    // filter down to the authentication providers
                    .filter(|(u, _)| AUTHENTICATOR_ACCOUNT_TYPES.contains(u))
                    // Collect into map.
                    .collect();

            // If there is not a secondary account for the user to authenticate with,
            // return an error.
            if all_accounts.len() <= 1 {
                return Err(TelescopeError::BadRequest {
                    header: format!("Cannot unlink {} account", Self::USER_ACCOUNT_TY),
                    message: "You have no other authentication methods linked, so unlinking \
                    this platform would prevent you from logging in."
                        .into(),
                    show_status_code: false,
                });
            }

            // There is a secondary authenticator linked, delete this user account record.
            // Log a message about the unlinked platform.
            let platform_id =
                UnlinkUserAccount::send(username.clone(), Self::USER_ACCOUNT_TY).await?;

            if let Some(platform_id) = platform_id {
                info!(
                    "User {} unlinked {} account with id {}.",
                    username,
                    Self::USER_ACCOUNT_TY,
                    platform_id
                );
            }

            // Try to replace the unlinked account in the authentication cookie's root
            // (if it's authenticated as root).
            let removed_auth: bool = cookie.remove_platform(Self::USER_ACCOUNT_TY).await?;
            // If this is a success, then save the modified authentication cookie and redirect
            // the user to their profile.
            // If not, the user has been logged out. Redirect them to the homepage.
            if removed_auth {
                id.save(&cookie);
            } else {
                id.forget();
            }

            // Get the path to redirect the user to.
            let redirect: String = removed_auth
                // If the auth was replaced successfully, the user's profile.
                .then(|| profile_for(username.as_str()))
                // Otherwise the homepage.
                .unwrap_or("/".into());

            return Ok(HttpResponse::Found().header(LOCATION, redirect).finish());
        });
    }

    /// Actix-web handler for authentication callback to login. Guarded by this
    /// trait to GET requests.
    fn login_authenticated_handler(req: HttpRequest) -> Self::LoginAuthenticatedFut;

    /// Actix-web handler for authentication callback to account creation.
    /// Guarded by this trait to GET requests.
    fn registration_authenticated_handler(req: HttpRequest) -> Self::RegistrationAuthenticatedFut;

    /// Actix-web handler
    fn linking_authenticated_handler(
        req: HttpRequest,
        ident: Identity,
    ) -> Self::LinkAuthenticatedFut;
}
