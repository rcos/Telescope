use actix_web::{HttpRequest, HttpResponse};
use crate::error::TelescopeError;
use oauth2;
use crate::app_data::AppData;
use oauth2::basic::BasicClient;
use oauth2::{CsrfToken, RedirectUrl, Scope};
use actix_web::http::uri::Authority;
use std::borrow::Cow;

#[get("login/github")]
pub async fn login(req: HttpRequest) -> Result<HttpResponse, TelescopeError> {
    // Get request scheme and authority.
    let scheme: &str = req.uri().scheme_str().expect("Could not get request scheme string.");
    let authority: &Authority = req.uri().authority().expect("Could not get request authority.");
    // Create redirect URL.
    let redir_url: RedirectUrl = RedirectUrl::new(format!("{}://{}/auth/github", scheme, authority))
        .expect("Could not create GitHub OAuth2 Redirect URL");

    let auth_req = AppData::global()
        // Get the gloabal GitHub OAuth client.
        .github_oauth_client()
        // Create a new random CSRF token.
        .authorize_url(CsrfToken::new_random)
        // Add redirect URL.
        .set_redirect_url(Cow::Owned(redir_url))
        // Set scopes (https://docs.github.com/en/developers/apps/scopes-for-oauth-apps)
        // read:user is reading access to the user's profile data.
        .add_scope(Scope::new("read:user".into()))
        // user:email gives us read access to the user's email address.
        .add_scope(Scope::new("user:email".into()));


    Err(TelescopeError::NotImplemented)
}