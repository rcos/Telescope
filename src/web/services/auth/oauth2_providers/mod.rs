use super::{make_redirect_url, IdentityProvider};
use crate::api::rcos::users::accounts::for_user::UserAccounts;
use crate::api::rcos::users::accounts::link::LinkUserAccount;
use crate::api::rcos::users::accounts::unlink::UnlinkUserAccount;
use crate::api::rcos::users::UserAccountType;
use crate::api::rcos::{send_query, users::accounts::reverse_lookup};
use crate::error::TelescopeError;
use crate::web::services::auth::identity::{AuthenticationCookie, Identity, RootIdentity};
use crate::web::services::auth::AUTHENTICATOR_ACCOUNT_TYPES;
use crate::web::csrf;
use actix_web::http::header::LOCATION;
use actix_web::web::Query;
use actix_web::FromRequest;
use actix_web::{HttpRequest, HttpResponse};
use futures::future::LocalBoxFuture;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AuthorizationCode, AuthorizationRequest, CsrfToken, RedirectUrl, Scope};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use crate::api::rcos::users::accounts::reverse_lookup::ReverseLookup;

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

/// Trait for identity types provided by OAuth2 Identity Providers.
pub trait Oauth2Identity {
    /// The type of user account provided by this authentication cookie.
    const USER_ACCOUNT_TY: UserAccountType;

    /// Convert a basic token response into this identity type.
    fn from_basic_token(token: &BasicTokenResponse) -> Self;

    /// Get the on-platform user ID for the authenticated user.
    fn platform_user_id(&self) -> LocalBoxFuture<Result<String, TelescopeError>>;

    /// Create a root identity object from this platform identity.
    fn into_root(self) -> RootIdentity;

    /// Add this platform identity to the user's auth cookie.
    fn add_to_cookie(self, cookie: &mut AuthenticationCookie);
}

/// Special trait specifically for OAuth2 Identity providers that implements
/// certain methods in the IdentityProvider trait automatically.
pub trait Oauth2IdentityProvider {
    /// The type of identity produced by this provider.
    type IdentityType: Oauth2Identity;

    /// Name of this identity provider. See the documentation on the
    /// [`IdentityProvider`] trait for requirements.
    const SERVICE_NAME: &'static str;

    /// Get the client configuration for this Identity Provider.
    fn get_client() -> Arc<BasicClient>;

    /// Add the appropriate scopes for the OAuth authentication request.
    fn scopes() -> Vec<Scope>;

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
            .set_redirect_uri(Cow::Owned(redir_url));

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
                TelescopeError::BadRequest {
                    header: "Bad Authentication Request".into(),
                    message: format!(
                        "Could not get authentication parameters from request URL. \
                    Actix-web error: {}",
                        err
                    ),
                    show_status_code: true,
                }
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
    const USER_ACCOUNT_TY: UserAccountType =
        <Self as Oauth2IdentityProvider>::IdentityType::USER_ACCOUNT_TY;

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
            // Get the API access token.
            let token_response: BasicTokenResponse = Self::token_exchange(redir_uri, &req)?;
            // Into the platform identity.
            let platform_identity: T::IdentityType =
                T::IdentityType::from_basic_token(&token_response);
            // Into a root identity.
            let root: RootIdentity = platform_identity.into_root();
            // Get the on-platform ID of the user's identity.
            let platform_id: String = root.get_platform_id().await?;

            // Send API query.
            let user_id = ReverseLookup::execute(root.get_user_account_type(), platform_id)
                .await?
                .ok_or(TelescopeError::resource_not_found(
                    "Could not find associated user account.",
                    format!("Could not find user account associated with this {} account. \
                    Please create an account or sign in using another method.",
                    Self::SERVICE_NAME)))?;

            // Otherwise, store the identity in the user's cookies and redirect to their profile.
            let identity: Identity = Identity::extract(&req).await?;
            identity.save(&root.make_authenticated_cookie());
            Ok(HttpResponse::Found()
                .header(LOCATION, format!("/user/{}", user_id))
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
            let platform_identity: T::IdentityType =
                T::IdentityType::from_basic_token(&token_response);
            let root: RootIdentity = platform_identity.into_root();

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
        ident: Identity,
    ) -> Self::LinkAuthenticatedFut {
        return Box::pin(async move {
            // Get the redirect url.
            let redir_url: RedirectUrl = make_redirect_url(&req, Self::link_redirect_path());
            // Token exchange.
            let token: BasicTokenResponse = Self::token_exchange(redir_url, &req)?;

            // Extract the auth cookie from the identity.
            let mut cookie: AuthenticationCookie = ident
                .identity()
                .await
                .ok_or(TelescopeError::NotAuthenticated)?;

            // Make platform identity.
            let platform_identity: T::IdentityType = T::IdentityType::from_basic_token(&token);

            // Get the platform ID.
            let platform_id: String = platform_identity.platform_user_id().await?;

            // Add/update user account record in the RCOS database.
            // First get the authenticated user's ID.
            let user_id = cookie.get_user_id_or_error().await?;

            info!(
                "Linking {} account ID {} to Telescope User {}",
                Self::USER_ACCOUNT_TY,
                platform_id,
                user_id
            );

            // Check if there is already an account of this type linked.
            // Lookup all linked accounts.
            let linked_accounts = UserAccounts::send(user_id)
                .await?
                .into_iter()
                .collect::<HashMap<UserAccountType, String>>();

            // If there is already an account for this service linked:
            if linked_accounts.contains_key(&Self::USER_ACCOUNT_TY) {
                // If the same account ID is linked, add to cookie and return.
                if linked_accounts[&Self::USER_ACCOUNT_TY] == platform_id {
                    info!("Already linked. Updating Cookie.");
                    // Add identity to auth cookie.
                    platform_identity.add_to_cookie(&mut cookie);
                    ident.save(&cookie);

                    // Return user to their profile.
                    return Ok(HttpResponse::Found()
                        .header(LOCATION, format!("/user/{}", user_id))
                        .finish());
                }

                // Otherwise try to replace the linked account.
                // Make sure another authenticator account is linked first.
                let other_authenticator_accounts: usize = linked_accounts
                    .iter()
                    .filter(|(ty, _)| **ty != Self::USER_ACCOUNT_TY)
                    .filter(|(ty, _)| AUTHENTICATOR_ACCOUNT_TYPES.contains(ty))
                    .count();

                // If there are other authenticated accounts, we can remove this one before
                // sending the link mutation.
                if other_authenticator_accounts >= 1 {
                    info!("Replacing currently linked account.");

                    // Send unlink mutation.
                    UnlinkUserAccount::send(user_id, Self::USER_ACCOUNT_TY).await?;
                }
            }

            // Send the link mutation.
            LinkUserAccount::send(user_id, Self::USER_ACCOUNT_TY, platform_id).await?;

            // Add identity to auth cookie.
            platform_identity.add_to_cookie(&mut cookie);
            ident.save(&cookie);

            // Redirect the user to their profile page
            Ok(HttpResponse::Found()
                .header(LOCATION, format!("/user/{}", user_id))
                .finish())
        });
    }
}
