//! Discord OAuth2 flow.

use crate::web::services::auth::oauth2_providers::Oauth2IdentityProvider;
use oauth2::{AuthorizationRequest, AccessToken, RefreshToken, TokenResponse, Scope};
use std::sync::Arc;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use chrono::{DateTime, Utc, Duration};
use crate::web::services::auth::identity::IdentityCookie;
use crate::env::global_config;
use oauth2::{AuthUrl, TokenUrl};

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

lazy_static!{
    static ref DISCORD_CLIENT: Arc<BasicClient> = {
        // Get the global config.
        let config = global_config();

        // Create GitHub OAuth2 client.
        let client = BasicClient::new(
            config.discord_config.client_id.clone(),
            Some(config.discord_config.client_secret.clone()),
            AuthUrl::new("https://discord.com/api/oauth2/authorize".into())
                .expect("Invalid Discord Auth URL"),
            Some(TokenUrl::new("https://discord.com/api/oauth2/token".into())
                .expect("Invalid Discord Token URL")));

        // Return the client config wrapped in an Arc.
        Arc::new(client)
    };
}

impl Oauth2IdentityProvider for DiscordOAuth {
    const SERVICE_NAME: &'static str = "discord";

    fn get_client() -> Arc<BasicClient> {
        DISCORD_CLIENT.clone()
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
