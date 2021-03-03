//! Discord OAuth2 flow.

use crate::env::global_config;
use crate::error::TelescopeError;
use crate::web::services::auth::identity::IdentityCookie;
use crate::web::services::auth::oauth2_providers::Oauth2IdentityProvider;
use crate::web::services::auth::IdentityProvider;
use actix_web::http::header::ACCEPT;
use chrono::{DateTime, Duration, Utc};
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AccessToken, RefreshToken, Scope, TokenResponse};
use oauth2::{AuthUrl, TokenUrl};
use serenity::model::user::CurrentUser;
use std::sync::Arc;

/// The Discord API endpoint to query for user data.
const DISCORD_API_ENDPOINT: &'static str = "https://discord.com/api/v8";

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

lazy_static! {
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

    fn scopes() -> Vec<Scope> {
        vec![
            // Scope required for us to get the users identity.
            Scope::new("identify".to_owned()),
        ]
    }

    fn make_identity(token_response: &BasicTokenResponse) -> IdentityCookie {
        IdentityCookie::Discord(DiscordIdentity::from_response(token_response))
    }
}

impl DiscordIdentity {
    fn from_response(token_response: &BasicTokenResponse) -> Self {
        // Unwrap the token duration.
        let token_duration = token_response
            .expires_in()
            .expect("Discord did not return token duration.");
        // Convert the token duration to a chrono duration.
        let chrono_duration =
            Duration::from_std(token_duration).expect("Token duration out of range.");

        DiscordIdentity {
            access_token: token_response.access_token().clone(),
            expiration: Utc::now() + chrono_duration,
            refresh_token: token_response
                .refresh_token()
                .expect("Discord did not return refresh token.")
                .clone(),
        }
    }

    /// Refresh this access token if necessary.
    pub fn refresh(self) -> Result<Self, TelescopeError> {
        // If this token has expired
        if self.expiration < Utc::now() {
            // Get a discord client and make a refresh token request.
            let client: Arc<BasicClient> = <DiscordOAuth as Oauth2IdentityProvider>::get_client();
            let mut refresh_token_request = client.exchange_refresh_token(&self.refresh_token);
            // Add scopes.
            for scope in DiscordOAuth::scopes() {
                refresh_token_request = refresh_token_request.add_scope(scope);
            }
            // Create refresh response
            let response = refresh_token_request
                // Add login redirect path.
                .add_extra_param("redirect_uri", DiscordOAuth::login_redirect_path().as_str())
                // Send the request. (This is synchronous -- be careful).
                .request(oauth2::reqwest::http_client)
                // Handle and propagate the error.
                .map_err(|err| {
                    TelescopeError::ise(format!(
                        "Could not refresh Discord OAuth2 token. Error: {}",
                        err
                    ))
                })?;

            // Make and return the new token.
            return Ok(Self::from_response(&response));
        } else {
            // We don't need to refresh -- return self.
            return Ok(self);
        }
    }

    /// Get the user authenticated in association with this access token. Assume this token has been
    /// refreshed recently enough.
    pub async fn authenticated_user(&self) -> Result<CurrentUser, TelescopeError> {
        // Make a web client to request the user's identity from the discord API.
        let client = actix_web::client::Client::builder()
            .bearer_auth(self.access_token.secret())
            .header(ACCEPT, "application/json")
            .finish();

        // Send the GET request to the discord API.
        return client
            .get(format!("{}/users/@me", DISCORD_API_ENDPOINT))
            .send()
            .await
            .map_err(TelescopeError::api_query_error)?
            .json::<CurrentUser>()
            .await
            .map_err(TelescopeError::api_response_error);
    }

    /// Get the authenticated Discord account's ID.
    pub async fn get_user_id(&self) -> Result<String, TelescopeError> {
        self.authenticated_user().await.map(|u| u.id.to_string())
    }
}
