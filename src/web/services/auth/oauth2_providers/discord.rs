//! Discord OAuth2 flow.

use std::sync::Arc;

use crate::api::rcos::send_query;
use crate::api::rcos::users::accounts::reverse_lookup::ReverseLookup;
use crate::api::rcos::users::UserAccountType;
use crate::env::global_config;
use crate::error::TelescopeError;
use crate::web::services::auth::identity::{AuthenticationCookie, RootIdentity};
use crate::web::services::auth::oauth2_providers::{Oauth2Identity, Oauth2IdentityProvider};
use crate::web::services::auth::IdentityProvider;
use actix_web::http::header::ACCEPT;
use chrono::{DateTime, Duration, Utc};
use futures::future::LocalBoxFuture;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AccessToken, RefreshToken, Scope, TokenResponse};
use oauth2::{AuthUrl, TokenUrl};
use serenity::model::id::RoleId;
use serenity::model::user::CurrentUser;

/// The Discord API endpoint to query for user data.
pub const DISCORD_API_ENDPOINT: &'static str = "https://discord.com/api/v8";

/// Zero-sized type used to represent Discord based identity verification.
pub struct DiscordOAuth;

/// The object stored in a user's cookies when authenticated via discord.
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    type IdentityType = DiscordIdentity;
    const SERVICE_NAME: &'static str = "discord";

    fn get_client() -> Arc<BasicClient> {
        DISCORD_CLIENT.clone()
    }

    fn scopes() -> Vec<Scope> {
        vec![
            // Scope required for us to get the users identity.
            Scope::new("identify".to_string()),
            // Scope required for us to add users to the RCOS Discord server.
            Scope::new("guilds.join".to_string()),
        ]
    }
}

impl Oauth2Identity for DiscordIdentity {
    const USER_ACCOUNT_TY: UserAccountType = UserAccountType::Discord;

    fn from_basic_token(token: &BasicTokenResponse) -> Self {
        Self::from_response(token)
    }

    fn platform_user_id(&self) -> LocalBoxFuture<Result<String, TelescopeError>> {
        Box::pin(async move { self.get_user_id().await })
    }

    fn into_root(self) -> RootIdentity {
        RootIdentity::Discord(self)
    }

    fn add_to_cookie(self, cookie: &mut AuthenticationCookie) {
        cookie.discord = Some(self);
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
    pub async fn refresh(self) -> Result<Self, TelescopeError> {
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
                // Send the request.
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

    /// Get the authenticated Discord account's ID.
    pub async fn get_user_id(&self) -> Result<String, TelescopeError> {
        self.get_authenticated_user()
            .await
            .map(|u| u.id.to_string())
    }

    /// Get the RCOS username of the account associated with the authenticated
    /// discord user if one exists.
    pub async fn get_rcos_username(&self) -> Result<Option<String>, TelescopeError> {
        // Get the authenticated user id.
        let platform_id: String = self.get_user_id().await?;
        // Build the query variables for a reverse lookup query to the central RCOS API
        let variables = ReverseLookup::make_vars(UserAccountType::Discord, platform_id);
        // Send the query and await the response
        return send_query::<ReverseLookup>(variables)
            .await
            .map(|response| response.username());
    }

    /// Get the currently authenticated discord user associated with this access token.
    pub async fn get_authenticated_user(&self) -> Result<CurrentUser, TelescopeError> {
        // Send the GET request to the discord API.
        return reqwest::Client::new()
            .get(format!("{}/users/@me", DISCORD_API_ENDPOINT).as_str())
            .bearer_auth(self.access_token.secret())
            .header(ACCEPT, "application/json")
            .send()
            .await
            .map_err(|e| {
                TelescopeError::ise(format!(
                    "Could not send identification query to Discord \
            API. Internal error: {}",
                    e
                ))
            })?
            .json::<CurrentUser>()
            .await
            .map_err(|e| {
                TelescopeError::ise(format!(
                    "Error with identification response from Discord \
            API. Internal error: {}",
                    e
                ))
            });
    }

    /// Add this user to the RCOS Discord. Set their username and
    pub async fn add_to_rcos_guild(
        &self,
        nickname: Option<String>,
        roles: Vec<RoleId>,
    ) -> Result<(), TelescopeError> {
        // Get user ID.
        let user_id: String = self.get_user_id().await?;
        // Get the RCOS Discord server ID.
        let rcos_discord = &global_config().discord_config.rcos_guild_id;
        // Make the request URL.
        let url: String = format!(
            "{}/guilds/{}/members/{}",
            DISCORD_API_ENDPOINT, rcos_discord, user_id
        );
        // Make the request object (JSON sent to Discord).
        let body = json!({
            "access_code": self.access_token.secret(),
            "nick": nickname,
            "roles": roles
        });

        // Send Discord request.
        let _ = reqwest::Client::new()
            .put(url.as_str())
            .json(&body)
            .bearer_auth(global_config().discord_config.bot_token.as_str())
            .send()
            .await
            .map_err(|err| {
                error!("Could not add user to RCOS Discord. Reqwest error: {}", err);
                TelescopeError::ise(format!(
                    "Could not join RCOS Discord. Internal Error: {}",
                    err
                ))
            })?;

        return Ok(());
    }
}
