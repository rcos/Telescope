//! Discord OAuth2 flow.

use crate::web::services::auth::oauth2_providers::Oauth2IdentityProvider;
use oauth2::{AuthorizationRequest, AccessToken, RefreshToken, TokenResponse, Scope};
use std::sync::Arc;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use chrono::{DateTime, Utc, Duration};
use crate::web::services::auth::identity::IdentityCookie;

/// Zero-sized type used to represent Discord based identity verification.
pub struct DiscordOAuth;

/// The object stored in a user's cookies when authenticated via discord.
#[derive(Serialize, Deserialize)]
pub struct DiscordIdentity {
    /// The OAuth2 access token granted by discord.
    access_token: AccessToken,
    /// When the access token expires.
    expiration: DateTime<Utc>,
    /// The token to use to refresh it.
    refresh_token: RefreshToken,
}

impl Oauth2IdentityProvider for DiscordOAuth {
    const SERVICE_NAME: &'static str = "discord";

    fn get_client() -> Arc<BasicClient> {
        unimplemented!()
    }

    fn add_scopes(auth_req: AuthorizationRequest) -> AuthorizationRequest {
        auth_req
            // Scope required for us to get the users identity.
            .add_scope(Scope::new("identify".to_owned()))
    }

    fn make_identity(token_response: &BasicTokenResponse) -> IdentityCookie {
        let token_duration = token_response.expires_in()
            .expect("Discord did not return token duration.");
        let chrono_duration = Duration::from_std(token_duration)
            .expect("Token duration out of range.");

        IdentityCookie::Discord(DiscordIdentity {
            access_token: token_response.access_token().clone(),
            expiration: Utc::now() + chrono_duration,
            refresh_token: token_response.refresh_token()
                .expect("Discord did not return refresh token.")
                .clone()
        })
    }
}
