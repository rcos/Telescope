use super::{make_redirect_url, IdentityProvider};
use crate::error::TelescopeError;
use crate::web::csrf;
use actix_web::http::header::LOCATION;
use actix_web::{HttpRequest, HttpResponse};
use futures::future::LocalBoxFuture;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AuthorizationRequest, CsrfToken, RedirectUrl, AuthorizationCode};
use std::borrow::Cow;
use std::sync::Arc;
use actix_web::web::Query;
use actix_web::FromRequest;
use std::future::Future;
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::web::services::auth::identity::IdentityCookie;

pub mod github;
pub mod discord;

/// Data returned by GitHub OAuth2 Authorization request.
#[derive(Deserialize)]
struct AuthResponse {
    /// The auth code.
    code: AuthorizationCode,
    /// The CSRF token. This should match the one that I sent them and stored
    /// in the CSRF table.
    state: CsrfToken,
}

/// Special trait specifically for OAuth2 Identity providers that implements
/// certain methods in the IdentityProvider trait automatically.
pub trait Oauth2IdentityProvider {
    /// Name of this identity provider. See the documentation on the
    /// [`IdentityProvider`] trait for requirements.
    const SERVICE_NAME: &'static str;

    /// Get the client configuration for this Identity Provider.
    fn get_client() -> Arc<BasicClient>;

    /// Add the appropriate scopes for the OAuth authentication request.
    fn add_scopes(auth_req: AuthorizationRequest) -> AuthorizationRequest;

    /// Create a user identity struct from an auth token response to save
    /// in the user's cookies and identify them in future requests.
    fn make_identity(token_response: &BasicTokenResponse) -> IdentityCookie;

    /// Get the redirect URL for the associated client and build an HTTP response to take the user
    /// there. Saves the CSRF token in the process.
    fn auth_response(
        redir_url: RedirectUrl,
        http_req: &HttpRequest,
    ) -> Result<HttpResponse, TelescopeError> {
        // Get the client configuration and build out the authentication request parameters.
        let client: Arc<BasicClient> = Self::get_client();
        let auth_req: AuthorizationRequest = client
            // Randomly generate a CSRF token.
            .authorize_url(CsrfToken::new_random)
            // Add the redirect URL.
            .set_redirect_url(Cow::Owned(redir_url));

        // Add the scopes defined by this Identity provider and convert the
        // request into the target URL and assocated CSRF token.
        let (url, csrf_token) = Self::add_scopes(auth_req).url();

        // Save CSRF token.
        csrf::save(Self::SERVICE_NAME, http_req, csrf_token)?;

        // Return the URL in an HTTP redirect response.
        return Ok(HttpResponse::Found()
            .header(LOCATION, url.as_str())
            .finish());
    }

    /// Extract the response parameters from the callback request invoked
    /// by the provider's authorization page.
    fn token_exchange(req: &HttpRequest) -> Result<BasicTokenResponse, TelescopeError> {
        // Extract the parameters from the request.
        let params: Query<AuthResponse> = Query::extract(req)
            // Extract the value out of the immediately ready future.
            .into_inner()
            // Propagate any errors that occur.
            .map_err(|err: actix_web::Error| {
                // Map all errors getting the query from the request into a bad
                // request error.
                TelescopeError::bad_request(
                    "Bad Authentication Request",
                    format!("Could not get authentication parameters from request URL. \
                    Actix-web error: {}", err)
                )
            })?;

        // Destructure the parameters.
        let AuthResponse {code, state} = params.0;
        // Verify the CSRF token. Propagate any errors including a mismatch
        // (we expect to verify without issue most of the time).
        csrf::verify(Self::SERVICE_NAME, req, state)?;

        // Get the OAuth2 client to exchange the auth code for an access token.
        let oauth_client: Arc<BasicClient> = Self::get_client();

        // Send the exchange request and wait for a response. This happens
        // synchronously so take care where you call this function from.
        // Return the response to the calling function.
        return oauth_client
            .exchange_code(code)
            // Send request and wait for response synchronously.
            .request(oauth2::reqwest::http_client)
            // Any errors that occur should be reported as internal server errors.
            .map_err(|e|
                TelescopeError::ise(format!("OAuth2 token exchange error. If this \
                persists, please contact a coordinator and file a GitHub issue. Internal error \
                description: {:?}", e)))
    }
}

impl<T> IdentityProvider for T
where
    T: Oauth2IdentityProvider + 'static,
{
    type Client = Arc<BasicClient>;

    fn get_client() -> Self::Client {
        <Self as Oauth2IdentityProvider>::get_client()
    }

    const SERVICE_NAME: &'static str = <Self as Oauth2IdentityProvider>::SERVICE_NAME;

    type LoginFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type RegistrationFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type LoginAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type RegistrationAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;

    fn login_handler(req: HttpRequest) -> Self::LoginFut {
        return Box::pin(async move {
            // Get the redirect URL.
            let redir_url: RedirectUrl = make_redirect_url(&req, Self::login_redirect_path());
            // Redirect the user.
            return Self::auth_response(redir_url, &req);
        });
    }

    fn registration_handler(req: HttpRequest) -> Self::RegistrationFut {
        return Box::pin(async move {
            // Get the redirect URL.
            let redir_url: RedirectUrl =
                make_redirect_url(&req, Self::registration_redirect_path());
            // Redirect the user.
            return Self::auth_response(redir_url, &req);
        });
    }

    fn login_authenticated_handler(req: HttpRequest) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>> {
        return Box::pin(async move {
            // Get the API access token.
            let token_response: BasicTokenResponse = Self::token_exchange(&req)?;
            let cookie_identity: IdentityCookie = Self::make_identity(&token_response);

            Err::<HttpResponse, TelescopeError>(TelescopeError::NotImplemented)
        });
    }

    fn registration_authenticated_handler(req: HttpRequest) -> Self::RegistrationAuthenticatedFut {
        return Box::pin(async move {
            Err(TelescopeError::NotImplemented)
        });
    }
}
