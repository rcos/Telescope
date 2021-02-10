use super::{
    IdentityProvider,
    make_redirect_url
};
use oauth2::basic::BasicClient;
use std::sync::Arc;
use actix_web::{HttpRequest, HttpResponse};
use futures::future::LocalBoxFuture;
use crate::error::TelescopeError;
use oauth2::{RedirectUrl, CsrfToken, AuthorizationRequest};
use std::borrow::Cow;
use crate::web::csrf;
use actix_web::http::header::LOCATION;

pub mod github;

/// Special trait specifically for OAuth2 Identity providers that implements
/// certain methods in the IdentityProvider trait automatically.
trait Oauth2IdentityProvider {
    /// Name of this identity provider. See the documentation on the
    /// [`IdentityProvider`] trait for requirements.
    const SERVICE_NAME: &'static str;

    /// Get the client configuration for this Identity Provider.
    fn get_client() -> Arc<BasicClient>;

    /// Add the appropriate scopes for the OAuth authentication request.
    fn add_scopes(auth_req: AuthorizationRequest) -> AuthorizationRequest;
}


impl<T> IdentityProvider for T
where T: Oauth2IdentityProvider + 'static {
    type Client = Arc<BasicClient>;

    fn get_client() -> Self::Client { <Self as Oauth2IdentityProvider>::get_client() }

    const SERVICE_NAME: &'static str = <Self as Oauth2IdentityProvider>::SERVICE_NAME;

    fn registration_handler(req: HttpRequest) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>> {
        return Box::pin(async move {
            Err(TelescopeError::NotImplemented)
        });
    }

    fn login_handler(req: HttpRequest) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>> {
        return Box::pin(async move {
            // Get the base redirect URL build into the OAuth2 authentication request.
            let redir_url: RedirectUrl = make_redirect_url(&req, Self::login_redirect_path());

            // Get the client configuration and build out the authentication request parameters.
            let client = Self::get_client();
            let mut auth_req: AuthorizationRequest = client
                // Randomly generate a CSRF token.
                .authorize_url(CsrfToken::new_random)
                // Add the redirect URL.
                .set_redirect_url(Cow::Owned(redir_url));

            // Add the scopes defined by this Identity provider and convert the
            // request into the target URL and assocated CSRF token.
            let (url, csrf_token) = Self::add_scopes(auth_req).url();

            // Save CSRF token.
            csrf::save(Self::SERVICE_NAME, &req, csrf_token)?;

            // Return an HTTP Response bringing the user to the appropriate
            // authentication page.
            return Ok(HttpResponse::Found().header(LOCATION, url.as_str()).finish());
        });
    }

    fn login_authenticated_handler(req: HttpRequest) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>> {
        return Box::pin(async move {
            Err(TelescopeError::NotImplemented)
        });
    }

    fn registration_authenticated_handler(req: HttpRequest) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>> {
        return Box::pin(async move {
            Err(TelescopeError::NotImplemented)
        });
    }
}