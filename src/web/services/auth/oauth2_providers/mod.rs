use super::{make_redirect_url, IdentityProvider};
use crate::error::TelescopeError;
use crate::web::csrf;
use actix_web::http::header::LOCATION;
use actix_web::{HttpRequest, HttpResponse};
use futures::future::LocalBoxFuture;
use oauth2::basic::BasicClient;
use oauth2::{AuthorizationRequest, CsrfToken, RedirectUrl};
use std::borrow::Cow;
use std::sync::Arc;

pub mod github;

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

    /// Get the redirect URL for the associated client and build an HTTP response to take the user
    /// there. Saves the CSRF token in the process.
    fn auth_response(
        redir_url: RedirectUrl,
        http_req: &HttpRequest,
    ) -> Result<HttpResponse, TelescopeError> {
        // Get the client configuration and build out the authentication request parameters.
        let client = Self::get_client();
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

    // The registration handler is almost identical to the login handler but not much can be
    // factored out unfortunately.

    fn registration_handler(
        req: HttpRequest,
    ) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>> {
        return Box::pin(async move {
            // Get the redirect URL.
            let redir_url: RedirectUrl =
                make_redirect_url(&req, Self::registration_redirect_path());
            // Redirect the user.
            return Self::auth_response(redir_url, &req);
        });
    }

    fn login_handler(
        req: HttpRequest,
    ) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>> {
        return Box::pin(async move {
            // Get the redirect URL.
            let redir_url: RedirectUrl = make_redirect_url(&req, Self::login_redirect_path());
            // Redirect the user.
            return Self::auth_response(redir_url, &req);
        });
    }

    fn login_authenticated_handler(
        req: HttpRequest,
    ) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>> {
        return Box::pin(async move { Err(TelescopeError::NotImplemented) });
    }

    fn registration_authenticated_handler(
        req: HttpRequest,
    ) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>> {
        return Box::pin(async move { Err(TelescopeError::NotImplemented) });
    }
}
