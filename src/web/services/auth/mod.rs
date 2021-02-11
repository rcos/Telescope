use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::uri::Authority;
use actix_web::web as aweb;
use actix_web::web::ServiceConfig;
use futures::future::LocalBoxFuture;
use oauth2::RedirectUrl;
use crate::error::TelescopeError;
use oauth2_providers::github::GitHubOauth;

pub mod oauth2_providers;
pub mod rpi_cas;

/// Register auth services.
pub fn register(config: &mut ServiceConfig) {
    // GitHub OAuth2 provider services.
    GitHubOauth::register_services(config);

}

/// Function to create the redirect URL for a given request and identity provider's
/// redirect path.
fn make_redirect_url(req: &HttpRequest, redir_path: String) -> RedirectUrl {
    // Get request scheme and authority.
    let scheme: &str = req.uri().scheme_str().expect("Could not get request scheme string.");
    let authority: &Authority = req.uri().authority().expect("Could not get request authority.");
    // Create and return redirect URL.
    return RedirectUrl::new(format!("{}://{}{}", scheme, authority, redir_path))
        .expect("Could not create GitHub OAuth2 Redirect URL");
}

/// Trait for identity providers (GitHub OAuth2, Discord OAuth2, RPI CAS, etc).
#[async_trait]
pub trait IdentityProvider: 'static {
    /// The client configuration type that stores information about the identity
    /// provider including the authorization URL and token URL for OAuth2
    /// providers.
    type Client;

    /// Function to get the client configuration used by this provider.
    fn get_client() -> Self::Client;

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
    /// this trait. This is similar to [`login_path`] but is for account
    /// registration rather than sign in.
    fn register_path() -> String {
        format!("/register/{}", Self::SERVICE_NAME)
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

    /// Register the necessary actix services to support this identity
    /// provider.
    fn register_services(config: &mut ServiceConfig) {
        config
            .route(Self::register_path().as_str(), aweb::get().to(Self::registration_handler))
            .route(Self::login_path().as_str(), aweb::get().to(Self::login_handler))
            .route(Self::login_redirect_path().as_str(),
                   aweb::get().to(Self::login_authenticated_handler))
            .route(Self::registration_redirect_path().as_str(),
                   aweb::get().to(Self::registration_authenticated_handler));
    }

    /// Actix-web handler for the route that redirects to authentication for
    /// account creation (user registration). Guarded by this
    /// trait to GET requests.
    fn registration_handler(req: HttpRequest) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;

    /// Actix-web handler for the route that redirects to authentication for
    /// login. Guarded by this trait to GET requests.
    fn login_handler(req: HttpRequest) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;

    /// Actix-web handler for authentication callback to login. Guarded by this
    /// trait to GET requests.
    fn login_authenticated_handler(req: HttpRequest)
        -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;

    /// Actix-web handler for authentication callback to account creation.
    /// Guarded by this trait to GET requests.
    fn registration_authenticated_handler(req: HttpRequest)
        -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
}


