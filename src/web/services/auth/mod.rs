use crate::error::TelescopeError;
use crate::web::services::auth::oauth2_providers::discord::DiscordOAuth;
use actix_web::http::header::HOST;
use actix_web::web as aweb;
use actix_web::web::ServiceConfig;
use actix_web::{HttpRequest, HttpResponse};
use oauth2::RedirectUrl;
use oauth2_providers::github::GitHubOauth;
use std::future::Future;
use crate::web::services::auth::rpi_cas::RpiCas;
use crate::web::services::auth::identity::Identity;

pub mod identity;
pub mod oauth2_providers;
pub mod rpi_cas;

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
    fn link_path() -> String { format!("/link/{}", Self::SERVICE_NAME) }

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
    fn link_redirect_path() -> String { format!("/auth/{}/link", Self::SERVICE_NAME) }

    /// The type of future returned by the login handler.
    type LoginFut: Future<Output = Result<HttpResponse, TelescopeError>> + 'static;

    /// The type of the future returned by the registration handler.
    type RegistrationFut: Future<Output = Result<HttpResponse, TelescopeError>> + 'static;

    /// The type of the future returned by the account linking handler.
    type LinkFut: Future<Output = Result<HttpResponse, TelescopeError>> + 'static;

    /// The type of future returned by the login authenticated response handler.
    type LoginAuthenticatedFut: Future<Output = Result<HttpResponse, TelescopeError>> + 'static;

    /// The type of future returned by the registration authenticated response handler.
    type RegistrationAuthenticatedFut: Future<Output = Result<HttpResponse, TelescopeError>> + 'static;

    /// The type of future returned by the registration authenticated response handler.
    type LinkAuthenticatedFut: Future<Output = Result<HttpResponse, TelescopeError>> + 'static;

    /// Register the necessary actix services to support this identity
    /// provider.
    fn register_services(config: &mut ServiceConfig) {
        config
            .route(Self::register_path().as_str(), aweb::get().to(Self::registration_handler))
            .route(Self::login_path().as_str(), aweb::get().to(Self::login_handler))
            .route(Self::link_path().as_str(), aweb::get().to(Self::link_handler))
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
                aweb::get().to(Self::linking_authenticated_handler));
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
    fn link_handler(req: HttpRequest, ident: Identity) -> Self::LinkFut;

    /// Actix-web handler for authentication callback to login. Guarded by this
    /// trait to GET requests.
    fn login_authenticated_handler(req: HttpRequest) -> Self::LoginAuthenticatedFut;

    /// Actix-web handler for authentication callback to account creation.
    /// Guarded by this trait to GET requests.
    fn registration_authenticated_handler(req: HttpRequest) -> Self::RegistrationAuthenticatedFut;

    /// Actix-web handler
    fn linking_authenticated_handler(req: HttpRequest, ident: Identity) -> Self::LinkAuthenticatedFut;
}
