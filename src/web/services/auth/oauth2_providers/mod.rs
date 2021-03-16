use super::{make_redirect_url, IdentityProvider};
use crate::error::TelescopeError;
use crate::web::api::rcos::{send_query, users::accounts::reverse_lookup};
use crate::web::csrf;
use crate::web::services::auth::identity::{Identity, RootIdentity};
use actix_web::http::header::LOCATION;
use actix_web::web::Query;
use actix_web::{FromRequest, Responder};
use actix_web::{HttpRequest, HttpResponse};
use futures::future::LocalBoxFuture;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AuthorizationCode, AuthorizationRequest, CsrfToken, RedirectUrl, Scope};
use std::borrow::Cow;
use std::sync::Arc;

pub mod discord;
pub mod github;

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
    fn scopes() -> Vec<Scope>;

    /// Create a user identity struct from an auth token response to save
    /// in the user's cookies and identify them in future requests.
    fn make_identity(token_response: &BasicTokenResponse) -> RootIdentity;

    /// Add the authenticated identity to a user's token.
    fn add_to_identity<'a>(
        token_response: &'a BasicTokenResponse,
        identity: &'a mut Identity,
    ) -> LocalBoxFuture<'a, Result<(), TelescopeError>>;

    /// Get the redirect URL for the associated client and build an HTTP response to take the user
    /// there. Saves the CSRF token in the process.
    fn auth_response(
        redir_url: RedirectUrl,
        http_req: &HttpRequest,
    ) -> Result<HttpResponse, TelescopeError> {
        // Get the client configuration and build out the authentication request parameters.
        let client: Arc<BasicClient> = Self::get_client();
        let mut auth_req: AuthorizationRequest = client
            // Randomly generate a CSRF token.
            .authorize_url(CsrfToken::new_random)
            // Add the redirect URL.
            .set_redirect_url(Cow::Owned(redir_url));

        // Add the scopes defined by this Identity provider and convert the
        // request into the target URL and assocated CSRF token.
        for scope in Self::scopes() {
            auth_req = auth_req.add_scope(scope);
        }
        let (url, csrf_token) = auth_req.url();

        // Save CSRF token.
        csrf::save(Self::SERVICE_NAME, http_req, csrf_token)?;

        // Return the URL in an HTTP redirect response.
        return Ok(HttpResponse::Found()
            .header(LOCATION, url.as_str())
            .finish());
    }

    /// Extract the response parameters from the callback request invoked
    /// by the provider's authorization page.
    fn token_exchange(
        redirect_uri: RedirectUrl,
        req: &HttpRequest,
    ) -> Result<BasicTokenResponse, TelescopeError> {
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
                    format!(
                        "Could not get authentication parameters from request URL. \
                    Actix-web error: {}",
                        err
                    ),
                )
            })?;

        // Destructure the parameters.
        let AuthResponse { code, state } = params.0;
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
            .add_extra_param("redirect_uri", redirect_uri.as_str())
            // Send request and wait for response synchronously.
            .request(oauth2::reqwest::http_client)
            // Any errors that occur should be reported as internal server errors.
            .map_err(|e| {
                TelescopeError::ise(format!(
                    "OAuth2 token exchange error. If this \
                persists, please contact a coordinator and file a GitHub issue. Internal error \
                description: {:?}",
                    e
                ))
            });
    }
}

impl<T> IdentityProvider for T
where
    T: Oauth2IdentityProvider + 'static,
{
    const SERVICE_NAME: &'static str = <Self as Oauth2IdentityProvider>::SERVICE_NAME;

    type LoginResponse = Result<HttpResponse, TelescopeError>;
    type RegistrationResponse = Result<HttpResponse, TelescopeError>;
    type LinkResponse = Result<HttpResponse, TelescopeError>;

    type LoginFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type RegistrationFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type LinkFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;

    type LoginAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type RegistrationAuthenticatedFut =
        LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type LinkAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;

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

    fn link_handler(req: HttpRequest, ident: Identity) -> Self::LinkFut {
        return Box::pin(async move {
            // Check that the user is already authenticated with another service.
            if ident.identity().await.is_some() {
                // If so, make the redirect url and send the user there.
                let redir_url: RedirectUrl = make_redirect_url(&req, Self::link_redirect_path());
                return Self::auth_response(redir_url, &req);
            } else {
                // If not, respond with a NotAuthenticated error.
                return Err(TelescopeError::NotAuthenticated);
            }
        });
    }

    fn login_authenticated_handler(
        req: HttpRequest,
    ) -> LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>> {
        return Box::pin(async move {
            // Get the redirect URL.
            let redir_uri: RedirectUrl = make_redirect_url(&req, Self::login_redirect_path());
            // Get the API access token (in an identity cookie).
            let token_response: BasicTokenResponse = Self::token_exchange(redir_uri, &req)?;
            let root: RootIdentity = Self::make_identity(&token_response);
            // Get the on-platform ID of the user's identity.
            let platform_id: String = root.get_platform_id().await?;

            // Make variables.
            let variables = reverse_lookup::reverse_lookup::Variables {
                id: platform_id,
                platform: root.get_user_account_type(),
            };
            // Send API query.
            let username: Option<String> =
                send_query::<reverse_lookup::ReverseLookup>(None, variables)
                    .await?
                    .username();

            // If there is no user, return a not-found error.
            let username: String = username.ok_or(TelescopeError::resource_not_found(
                "Could not find associated user account.",
                format!(
                    "Could not find user account associated with this {} account. \
                Please create an account or sign in using another method.",
                    Self::SERVICE_NAME
                ),
            ))?;

            // Otherwise, store the identity in the user's cookies and redirect to their profile.
            let identity: Identity = Identity::extract(&req).await?;
            identity.save(&root.make_authenticated_cookie());
            Ok(HttpResponse::Found()
                .header(LOCATION, format!("/user/{}", username))
                .finish())
        });
    }

    fn registration_authenticated_handler(req: HttpRequest) -> Self::RegistrationAuthenticatedFut {
        return Box::pin(async move {
            // Get the redirect URL.
            let redir_uri: RedirectUrl =
                make_redirect_url(&req, Self::registration_redirect_path());
            // Get the object to store in the user's cookie.
            let token_response: BasicTokenResponse = Self::token_exchange(redir_uri, &req)?;
            let root: RootIdentity = Self::make_identity(&token_response);
            // Extract the identity object from the request and store the cookie in it.
            let identity: Identity = Identity::extract(&req).await?;
            identity.save(&root.make_authenticated_cookie());

            // Success! Redirect the user to finish the registration process.
            Ok(HttpResponse::Found()
                .header(LOCATION, "/register/finish")
                .finish())
        });
    }

    fn linking_authenticated_handler(
        req: HttpRequest,
        mut ident: Identity,
    ) -> Self::LinkAuthenticatedFut {
        return Box::pin(async move {
            // Get the redirect url.
            let redir_url: RedirectUrl = make_redirect_url(&req, Self::link_redirect_path());
            // Token exchange.
            let token: BasicTokenResponse = Self::token_exchange(redir_url, &req)?;
            // Add to the user's identity.
            Self::add_to_identity(&token, &mut ident).await?;

            // Redirect the user to their profile page
            // Get the RCOS username
            let username: String = ident
                .get_rcos_username()
                .await?
                // Or respond with a not authenticated error.
                .ok_or(TelescopeError::NotAuthenticated)?;

            unimplemented!()
        });
    }
}
